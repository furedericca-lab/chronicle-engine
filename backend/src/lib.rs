pub mod admin;
pub mod config;
mod error;
pub mod models;
mod state;

pub use error::{AppError, AppResult};
pub use state::AppState;

use crate::{
    config::AppConfig,
    models::{
        validate_append_session_transcript_request, validate_delete_request,
        validate_enqueue_distill_job_request, validate_list_request,
        validate_recall_behavioral_request, validate_recall_generic_request,
        validate_stats_request, validate_store_request, validate_update_request, Actor,
        AppendSessionTranscriptRequest, DeleteRequest, EnqueueDistillJobRequest, HealthResponse,
        ListRequest, Principal, RecallBehavioralDebugResponse, RecallBehavioralRequest,
        RecallGenericDebugResponse, RecallGenericRequest, StatsRequest, StoreRequest,
        UpdateRequest,
    },
};
use axum::{
    body::Body,
    extract::{rejection::JsonRejection, Extension, Path, State},
    http::{header, HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::future::Future;

const AUTH_USER_ID_HEADER: &str = "x-auth-user-id";
const AUTH_AGENT_ID_HEADER: &str = "x-auth-agent-id";

#[derive(Clone, Debug)]
struct RuntimeAuthContext {
    principal: Principal,
}

pub fn build_app(config: AppConfig) -> anyhow::Result<Router> {
    config.validate()?;
    let state = AppState::new(config.clone())?;

    let data_routes = Router::new()
        .route("/recall/generic", post(recall_generic))
        .route("/recall/behavioral", post(recall_behavioral_guidance))
        .route("/debug/recall/generic", post(recall_generic_debug))
        .route(
            "/debug/recall/behavioral",
            post(recall_behavioral_guidance_debug),
        )
        .route("/memories/store", post(store_memories))
        .route(
            "/session-transcripts/append",
            post(append_session_transcript),
        )
        .route("/memories/update", post(update_memory))
        .route("/memories/delete", post(delete_memories))
        .route("/memories/list", post(list_memories))
        .route("/memories/stats", post(memory_stats))
        .route("/distill/jobs", post(enqueue_distill_job))
        .route("/distill/jobs/{job_id}", get(get_distill_job_status))
        .fallback(|| async { axum::http::StatusCode::NOT_FOUND })
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            runtime_auth_middleware,
        ));

    // Admin-plane rate limiter: 120 requests per 60 seconds per IP+token.
    let admin_rate_limiter = admin::rate_limit::AdminRateLimiter::new(120, 60);

    // Admin SPA shell: serve static assets from admin_assets_path, fallback to index.html
    let assets_path = config.server.admin_assets_path.clone();
    let index_path = assets_path.join("index.html");

    let serve_dir = tower_http::services::ServeDir::new(assets_path.clone());
    let serve_index = tower_http::services::ServeFile::new(index_path);

    let admin_api_routes = Router::new()
        .route("/health", get(admin::routes::admin_health))
        .route("/principals", get(admin::routes::list_principals))
        .route(
            "/principals/{principalId}/recall/simulate",
            post(admin::routes::recall_simulate),
        )
        .route(
            "/principals/{principalId}/memories",
            get(admin::routes::list_memories).post(admin::routes::create_memory),
        )
        .route(
            "/principals/{principalId}/memories/{memoryId}",
            get(admin::routes::get_memory)
                .patch(admin::routes::update_memory)
                .delete(admin::routes::delete_memory),
        )
        .route(
            "/principals/{principalId}/distill_jobs",
            get(admin::routes::list_distill_jobs)
        )
        .route(
            "/principals/{principalId}/distill_jobs/{jobId}",
            get(admin::routes::get_distill_job)
        )
        .route(
            "/principals/{principalId}/transcripts",
            get(admin::routes::list_transcripts)
        )
        .route(
            "/principals/{principalId}/transcripts/{transcriptId}",
            get(admin::routes::get_transcript)
        )
        .route(
            "/principals/{principalId}/governance",
            get(admin::routes::list_governance_artifacts)
        )
        .route(
            "/principals/{principalId}/governance/{artifactId}/review",
            post(admin::routes::review_governance_artifact)
        )
        .route(
            "/principals/{principalId}/governance/{artifactId}/promote",
            post(admin::routes::promote_governance_artifact)
        )
        .route(
            "/audit",
            get(admin::routes::get_audit_log)
        )
        .route(
            "/settings",
            get(admin::routes::get_settings)
                .post(admin::routes::update_settings)
        )
        .fallback(|| async { (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))) })
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(
            (
                config.auth.admin.token.clone(),
                config.auth.runtime.token.clone(),
                admin_rate_limiter,
            ),
            admin::auth::admin_auth_middleware,
        ));

    let admin_spa = Router::new()
        .nest_service("/assets", serve_dir)
        .fallback_service(axum::routing::get_service(serve_index));

    Ok(Router::new()
        .route("/v1/health", get(health))
        .nest("/v1", data_routes)
        .nest("/admin/api", admin_api_routes)
        .nest("/admin", admin_spa))
}

/// Initialize tracing/logging based on the logging.level config.
/// Call this before building the app in main.rs.
pub fn init_logging(level: &str) {
    use std::str::FromStr;
    let filter = tracing_subscriber::filter::LevelFilter::from_str(level)
        .unwrap_or(tracing_subscriber::filter::LevelFilter::INFO);
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(filter)
        .with_target(true)
        .with_thread_ids(false)
        .finish();
    // Ignore error if a global subscriber is already set (e.g. in tests).
    let _ = tracing::subscriber::set_global_default(subscriber);
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "memory-backend",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn recall_generic(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    payload: Result<Json<RecallGenericRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::RecallGenericResponse>> {
    let req = decode_json(payload)?;
    validate_recall_generic_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let rows = state.memory_repo.recall_generic(req).await?;
    Ok(Json(rows))
}

async fn recall_behavioral_guidance(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    payload: Result<Json<RecallBehavioralRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::RecallBehavioralResponse>> {
    let req = decode_json(payload)?;
    validate_recall_behavioral_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let rows = state.memory_repo.recall_behavioral_guidance(req).await?;
    Ok(Json(rows))
}

async fn recall_generic_debug(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    payload: Result<Json<RecallGenericRequest>, JsonRejection>,
) -> AppResult<Json<RecallGenericDebugResponse>> {
    let req = decode_json(payload)?;
    validate_recall_generic_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let (rows, trace) = state.memory_repo.recall_generic_with_trace(req).await?;
    Ok(Json(RecallGenericDebugResponse {
        rows: rows.rows,
        trace,
    }))
}

async fn recall_behavioral_guidance_debug(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    payload: Result<Json<RecallBehavioralRequest>, JsonRejection>,
) -> AppResult<Json<RecallBehavioralDebugResponse>> {
    let req = decode_json(payload)?;
    validate_recall_behavioral_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let (rows, trace) = state
        .memory_repo
        .recall_behavioral_guidance_with_trace(req)
        .await?;
    Ok(Json(RecallBehavioralDebugResponse {
        rows: rows.rows,
        trace,
    }))
}

async fn store_memories(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    headers: HeaderMap,
    payload: Result<Json<StoreRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::StoreResponse>> {
    let idempotency_key = require_idempotency_key(&headers)?.to_string();
    let req = decode_json(payload)?;
    validate_store_request(&req)?;
    ensure_actor_matches_context(req.actor(), &auth)?;
    let response = run_idempotent_operation(
        &state,
        &auth.principal,
        "POST /v1/memories/store",
        &idempotency_key,
        &fingerprint_request(&req)?,
        state.memory_repo.store(req.clone()),
    )
    .await?;

    let source_kind = match &req {
        StoreRequest::ToolStore { .. } => "tool-store",
        StoreRequest::AutoCapture { .. } => "auto-capture",
    };

    for res in &response.results {
        let prov = crate::models::MemoryProvenance {
            memory_id: res.id.clone(),
            source_kind: source_kind.to_string(),
            source_ref: None,
            source_label: None,
            source_detail_json: None,
            job_id: None,
            artifact_id: None,
            created_at: Some(crate::state::now_millis()),
        };
        // Best effort sync; failures are logged but don't fail the data plane response.
        if let Err(err) = state.job_store.save_memory_provenance(&prov) {
            tracing::error!("failed to save memory provenance: {}", err);
        }
    }

    Ok(Json(response))
}

async fn update_memory(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    headers: HeaderMap,
    payload: Result<Json<UpdateRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::UpdateResponse>> {
    let idempotency_key = require_idempotency_key(&headers)?.to_string();
    let req = decode_json(payload)?;
    validate_update_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let response = run_idempotent_operation(
        &state,
        &auth.principal,
        "POST /v1/memories/update",
        &idempotency_key,
        &fingerprint_request(&req)?,
        state.memory_repo.update(req),
    )
    .await?;
    Ok(Json(response))
}

async fn delete_memories(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    headers: HeaderMap,
    payload: Result<Json<DeleteRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::DeleteResponse>> {
    let idempotency_key = require_idempotency_key(&headers)?.to_string();
    let req = decode_json(payload)?;
    validate_delete_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let deleted = run_idempotent_operation(
        &state,
        &auth.principal,
        "POST /v1/memories/delete",
        &idempotency_key,
        &fingerprint_request(&req)?,
        state.memory_repo.delete(req.clone()),
    )
    .await?;

    if deleted > 0 {
        if let Some(ref mid) = req.memory_id {
            if let Err(err) = state.job_store.delete_memory_provenance(mid) {
                tracing::error!("failed to delete memory provenance: {}", err);
            }
        }
    }

    Ok(Json(crate::models::DeleteResponse { deleted }))
}

async fn append_session_transcript(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    headers: HeaderMap,
    payload: Result<Json<AppendSessionTranscriptRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::AppendSessionTranscriptResponse>> {
    let idempotency_key = require_idempotency_key(&headers)?.to_string();
    let req = decode_json(payload)?;
    validate_append_session_transcript_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let req_for_append = req.clone();
    let response = run_idempotent_operation(
        &state,
        &auth.principal,
        "POST /v1/session-transcripts/append",
        &idempotency_key,
        &fingerprint_request(&req)?,
        async { state.job_store.append_session_transcript(&req_for_append) },
    )
    .await?;
    Ok(Json(response))
}

async fn list_memories(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    payload: Result<Json<ListRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::ListResponse>> {
    let req = decode_json(payload)?;
    validate_list_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let response = state.memory_repo.list(req).await?;
    Ok(Json(response))
}

async fn memory_stats(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    payload: Result<Json<StatsRequest>, JsonRejection>,
) -> AppResult<Json<crate::models::StatsResponse>> {
    let req = decode_json(payload)?;
    validate_stats_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let response = state.memory_repo.stats(&req.actor).await?;
    Ok(Json(response))
}

async fn enqueue_distill_job(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    headers: HeaderMap,
    payload: Result<Json<EnqueueDistillJobRequest>, JsonRejection>,
) -> AppResult<(StatusCode, Json<crate::models::EnqueueDistillJobResponse>)> {
    let idempotency_key = require_idempotency_key(&headers)?.to_string();
    let req = decode_json(payload)?;
    validate_enqueue_distill_job_request(&req)?;
    ensure_actor_matches_context(&req.actor, &auth)?;
    let req_for_enqueue = req.clone();
    let response = run_idempotent_operation(
        &state,
        &auth.principal,
        "POST /v1/distill/jobs",
        &idempotency_key,
        &fingerprint_request(&req)?,
        async { state.job_store.enqueue_distill(&req_for_enqueue) },
    )
    .await?;
    let job_id = response.job_id.clone();
    let state_for_exec = state.clone();
    tokio::spawn(async move {
        let _ = state_for_exec.execute_distill_job(job_id, req).await;
    });
    Ok((StatusCode::ACCEPTED, Json(response)))
}

async fn get_distill_job_status(
    State(state): State<AppState>,
    Extension(auth): Extension<RuntimeAuthContext>,
    Path(job_id): Path<String>,
) -> AppResult<Json<crate::models::DistillJobStatusResponse>> {
    let status = state
        .job_store
        .get_scoped_distill(&job_id, &auth.principal.user_id, &auth.principal.agent_id)?
        .ok_or_else(|| AppError::not_found("distill job not found"))?;

    Ok(Json(status))
}

pub async fn runtime_auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    require_request_id(request.headers())?;
    let token = bearer_token(request.headers())?;
    if token != state.config.auth.runtime.token {
        return Err(AppError::unauthorized("invalid runtime bearer token"));
    }

    let principal = Principal {
        user_id: required_header(request.headers(), AUTH_USER_ID_HEADER)?.to_string(),
        agent_id: required_header(request.headers(), AUTH_AGENT_ID_HEADER)?.to_string(),
    };
    request
        .extensions_mut()
        .insert(RuntimeAuthContext { principal });

    Ok(next.run(request).await)
}

fn require_request_id(headers: &HeaderMap) -> AppResult<()> {
    let _ = required_header(headers, "x-request-id")?;
    Ok(())
}

pub(crate) fn require_idempotency_key(headers: &axum::http::HeaderMap) -> AppResult<&str> {
    required_header(headers, "idempotency-key")
}

fn required_header<'a>(headers: &'a HeaderMap, name: &str) -> AppResult<&'a str> {
    let value = headers
        .get(name)
        .ok_or_else(|| AppError::invalid_request(format!("missing required header: {name}")))?;
    let text = value
        .to_str()
        .map_err(|_| AppError::invalid_request(format!("invalid header value for {name}")))?;
    if text.trim().is_empty() {
        return Err(AppError::invalid_request(format!(
            "header {name} cannot be empty"
        )));
    }
    Ok(text)
}

fn bearer_token(headers: &HeaderMap) -> AppResult<String> {
    let value = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("missing Authorization header"))?
        .to_str()
        .map_err(|_| AppError::unauthorized("invalid Authorization header encoding"))?;
    let prefix = "Bearer ";
    if !value.starts_with(prefix) {
        return Err(AppError::unauthorized(
            "Authorization header must use Bearer scheme",
        ));
    }
    let token = value[prefix.len()..].trim();
    if token.is_empty() {
        return Err(AppError::unauthorized("Bearer token cannot be empty"));
    }
    Ok(token.to_string())
}

fn decode_json<T>(payload: Result<Json<T>, JsonRejection>) -> AppResult<T> {
    payload
        .map(|Json(value)| value)
        .map_err(|err| AppError::invalid_request(format!("invalid JSON request body: {err}")))
}

fn ensure_actor_matches_context(actor: &Actor, auth: &RuntimeAuthContext) -> AppResult<()> {
    if actor.user_id != auth.principal.user_id || actor.agent_id != auth.principal.agent_id {
        return Err(AppError::forbidden(
            "actor principal does not match authenticated request context",
        ));
    }
    Ok(())
}

pub(crate) async fn run_idempotent_operation<T, F>(
    state: &AppState,
    principal: &Principal,
    operation: &str,
    idempotency_key: &str,
    request_fingerprint: &str,
    action: F,
) -> AppResult<T>
where
    F: Future<Output = AppResult<T>>,
{
    let reservation = state.idempotency_store.reserve(
        principal,
        operation,
        idempotency_key,
        request_fingerprint,
    )?;
    match action.await {
        Ok(value) => {
            reservation.mark_completed()?;
            Ok(value)
        }
        Err(err) => {
            if let Err(mark_err) = reservation.mark_failed() {
                return Err(AppError::internal(format!(
                    "protected operation failed and idempotency state could not be marked failed; operation_error={err:?}; transition_error={mark_err:?}",
                )));
            }
            Err(err)
        }
    }
}

pub(crate) fn fingerprint_request<T: serde::Serialize>(request: &T) -> AppResult<String> {
    serde_json::to_string(request)
        .map_err(|err| AppError::internal(format!("failed to fingerprint request payload: {err}")))
}
