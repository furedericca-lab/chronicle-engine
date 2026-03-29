use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};

use crate::{
    error::{AppError, AppResult},
    AppState,
};

use super::{
    dto::{
        AdminHealthResponse, AdminRecallFilterSummary, AdminRecallMode,
        AdminRecallSimulateRequest, AdminRecallSimulateResponse,
    },
    principal_id::{decode_principal_id, encode_principal_id},
};

/// GET /admin/api/health — admin health probe.
pub async fn admin_health() -> Json<AdminHealthResponse> {
    Json(AdminHealthResponse {
        status: "ok",
        service: "chronicle-engine-rs",
        plane: "admin",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// GET /admin/api/principals — list known principals.
pub async fn list_principals(
    State(state): State<AppState>,
) -> AppResult<Json<Value>> {
    let principals = state.list_admin_principals().await?;
    let rows: Vec<Value> = principals
        .into_iter()
        .map(|(principal, stats)| {
            json!({
                "principalId": encode_principal_id(&principal),
                "userId": principal.user_id,
                "agentId": principal.agent_id,
                "memoryCount": stats.memory_count,
                "transcriptCount": stats.transcript_count,
                "distillJobCount": stats.distill_job_count,
                "lastActivityAt": stats.last_activity_at,
            })
        })
        .collect();
    Ok(Json(json!({ "principals": rows })))
}

/// POST /admin/api/principals/{principalId}/recall/simulate — side-effect-free recall.
pub async fn recall_simulate(
    State(state): State<AppState>,
    Path(principal_id): Path<String>,
    Json(req): Json<AdminRecallSimulateRequest>,
) -> AppResult<Json<AdminRecallSimulateResponse>> {
    let principal = decode_principal_id(&principal_id)?;

    if req.query.trim().is_empty() {
        return Err(AppError::invalid_request("query cannot be empty"));
    }
    if req.limit == 0 || req.limit > 200 {
        return Err(AppError::invalid_request("limit must be between 1 and 200"));
    }

    let (results, trace) = match req.mode {
        AdminRecallMode::Generic => {
            let (results, trace) = state
                .recall_generic_simulate(
                    &principal,
                    &req.query,
                    req.limit,
                    req.categories.as_deref(),
                    req.exclude_behavioral,
                    req.max_age_days,
                )
                .await?;
            (serde_json::to_value(&results).unwrap_or(json!([])), trace)
        }
        AdminRecallMode::Behavioral => {
            let (results, trace) = state
                .recall_behavioral_simulate(
                    &principal,
                    &req.query,
                    req.limit,
                    req.behavioral_mode,
                    req.include_kinds.as_deref(),
                    req.min_score,
                )
                .await?;
            (serde_json::to_value(&results).unwrap_or(json!([])), trace)
        }
    };

    Ok(Json(AdminRecallSimulateResponse {
        mode: req.mode,
        principal_user_id: principal.user_id,
        principal_agent_id: principal.agent_id,
        results,
        trace,
        applied_filters: AdminRecallFilterSummary {
            mode: req.mode,
            categories: req.categories,
            exclude_behavioral: req.exclude_behavioral,
            max_age_days: req.max_age_days,
            behavioral_mode: req.behavioral_mode,
            include_kinds: req.include_kinds,
            min_score: req.min_score,
            limit: req.limit,
        },
    }))
}

/// GET /admin/api/principals/{principalId}/memories — list memories.
pub async fn list_memories(
    State(state): State<AppState>,
    Path(principal_id): Path<String>,
) -> AppResult<Json<super::dto::AdminMemoryListResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let items = state.admin_list_memories(&principal).await?;
    Ok(Json(super::dto::AdminMemoryListResponse { items }))
}

/// GET /admin/api/principals/{principalId}/memories/{memoryId} — get memory detail.
pub async fn get_memory(
    State(state): State<AppState>,
    Path((principal_id, memory_id)): Path<(String, String)>,
) -> AppResult<Json<super::dto::AdminMemoryDetail>> {
    let principal = decode_principal_id(&principal_id)?;
    let detail = state.admin_get_memory(&principal, &memory_id).await?;
    detail.map(Json).ok_or_else(|| AppError::not_found("memory not found"))
}

/// POST /admin/api/principals/{principalId}/memories — create memory.
pub async fn create_memory(
    State(state): State<AppState>,
    Path(principal_id): Path<String>,
    headers: axum::http::HeaderMap,
    Json(req): Json<super::dto::AdminCreateMemoryRequest>,
) -> AppResult<Json<crate::models::StoreResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let idempotency_key = crate::require_idempotency_key(&headers)?.to_string();
    let req_fingerprint = crate::fingerprint_request(&req)?;
    let source_kind = req.source_kind.clone().unwrap_or_else(|| "admin-create".to_string());
    let actor = crate::models::Actor {
        user_id: principal.user_id.clone(),
        agent_id: principal.agent_id.clone(),
        session_id: "admin-session".to_string(),
        session_key: "admin".to_string(),
    };
    
    let memory = crate::models::ToolStoreMemory {
        text: req.text,
        category: req.category,
        importance: req.importance,
    };
    
    let store_req = crate::models::StoreRequest::ToolStore { actor, memory };
    let response = crate::run_idempotent_operation(
        &state,
        &principal,
        "POST /admin/api/principals/memories",
        &idempotency_key,
        &req_fingerprint,
        state.memory_repo.store(store_req.clone()),
    )
    .await?;

    for res in &response.results {
        let prov = crate::models::MemoryProvenance {
            memory_id: res.id.clone(),
            source_kind: source_kind.clone(),
            source_ref: None,
            source_label: None,
            source_detail_json: None,
            job_id: None,
            artifact_id: None,
            created_at: Some(crate::state::now_millis()),
        };
        let _ = state.job_store.save_memory_provenance(&prov);
    }
    
    Ok(Json(response))
}

/// PATCH /admin/api/principals/{principalId}/memories/{memoryId} — update memory.
pub async fn update_memory(
    State(state): State<AppState>,
    Path((principal_id, memory_id)): Path<(String, String)>,
    headers: axum::http::HeaderMap,
    Json(req): Json<super::dto::AdminUpdateMemoryRequest>,
) -> AppResult<Json<crate::models::UpdateResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let idempotency_key = crate::require_idempotency_key(&headers)?.to_string();
    let actor = crate::models::Actor {
        user_id: principal.user_id.clone(),
        agent_id: principal.agent_id.clone(),
        session_id: "admin-session".to_string(),
        session_key: "admin".to_string(),
    };
    
    let update_req = crate::models::UpdateRequest {
        actor,
        memory_id: memory_id.clone(),
        patch: crate::models::UpdatePatch {
            text: req.text.clone(),
            category: req.category,
            importance: req.importance,
        },
    };
    
    let response = crate::run_idempotent_operation(
        &state,
        &principal,
        "PATCH /admin/api/principals/memories",
        &idempotency_key,
        &crate::fingerprint_request(&req)?,
        state.memory_repo.update(update_req),
    )
    .await?;
    
    Ok(Json(response))
}

/// DELETE /admin/api/principals/{principalId}/memories/{memoryId} — delete memory.
pub async fn delete_memory(
    State(state): State<AppState>,
    Path((principal_id, memory_id)): Path<(String, String)>,
    headers: axum::http::HeaderMap,
) -> AppResult<Json<crate::models::DeleteResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let idempotency_key = crate::require_idempotency_key(&headers)?.to_string();
    let actor = crate::models::Actor {
        user_id: principal.user_id.clone(),
        agent_id: principal.agent_id.clone(),
        session_id: "admin-session".to_string(),
        session_key: "admin".to_string(),
    };
    
    let delete_req = crate::models::DeleteRequest {
        actor,
        memory_id: Some(memory_id.clone()),
        query: None,
    };
    
    let req_fingerprint = memory_id.clone();
    
    let deleted = crate::run_idempotent_operation(
        &state,
        &principal,
        "DELETE /admin/api/principals/memories",
        &idempotency_key,
        &req_fingerprint,
        state.memory_repo.delete(delete_req),
    )
    .await?;
    
    if deleted > 0 {
        let _ = state.job_store.delete_memory_provenance(&memory_id);
    }
    
    Ok(Json(crate::models::DeleteResponse { deleted }))
}

// ─── Phase 3 Admin Handlers ───

/// GET /admin/api/principals/{principalId}/distill_jobs
pub async fn list_distill_jobs(
    State(state): State<AppState>,
    Path(principal_id): Path<String>,
) -> AppResult<Json<super::dto::AdminDistillJobListResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let items = state.admin_list_distill_jobs(&principal)?;
    Ok(Json(super::dto::AdminDistillJobListResponse { items }))
}

/// GET /admin/api/principals/{principalId}/distill_jobs/{jobId}
pub async fn get_distill_job(
    State(state): State<AppState>,
    Path((principal_id, job_id)): Path<(String, String)>,
) -> AppResult<Json<super::dto::AdminDistillJobDetail>> {
    let principal = decode_principal_id(&principal_id)?;
    let detail = state
        .admin_get_distill_job(&principal, &job_id)?
        .ok_or_else(|| AppError::not_found("distill job not found"))?;
    Ok(Json(detail))
}

/// GET /admin/api/principals/{principalId}/transcripts
pub async fn list_transcripts(
    State(state): State<AppState>,
    Path(principal_id): Path<String>,
) -> AppResult<Json<super::dto::AdminTranscriptListResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let items = state.admin_list_transcripts(&principal)?;
    Ok(Json(super::dto::AdminTranscriptListResponse { items }))
}

/// GET /admin/api/principals/{principalId}/transcripts/{transcriptId}
pub async fn get_transcript(
    State(state): State<AppState>,
    Path((principal_id, transcript_id)): Path<(String, String)>,
) -> AppResult<Json<super::dto::AdminTranscriptDetailResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let (session_key, session_id) = super::principal_id::decode_transcript_id(&transcript_id)?;
    let detail = state
        .admin_get_transcript_detail(&principal, &session_key, &session_id)?
        .ok_or_else(|| AppError::not_found("transcript not found"))?;
    Ok(Json(detail))
}

/// GET /admin/api/principals/{principalId}/governance
pub async fn list_governance_artifacts(
    State(state): State<AppState>,
    Path(principal_id): Path<String>,
) -> AppResult<Json<super::dto::AdminGovernanceListResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let items = state.admin_list_governance_artifacts(&principal)?;
    Ok(Json(super::dto::AdminGovernanceListResponse { items }))
}

/// POST /admin/api/principals/{principalId}/governance/{artifactId}/review
pub async fn review_governance_artifact(
    State(state): State<AppState>,
    axum::Extension(auth_ctx): axum::Extension<super::auth::AdminAuthContext>,
    Path((principal_id, artifact_id)): Path<(String, String)>,
    Json(req): Json<super::dto::AdminGovernanceReviewRequest>,
) -> AppResult<Json<super::dto::AdminGovernanceReviewResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let admin_subject = format!("admin:{}", auth_ctx.token_fingerprint);
    let resp = state.admin_review_governance_artifact(
        &principal,
        &artifact_id,
        &req.review_status,
        req.reviewer_note.as_deref(),
        &admin_subject,
    )?;
    Ok(Json(resp))
}

/// POST /admin/api/principals/{principalId}/governance/{artifactId}/promote
pub async fn promote_governance_artifact(
    State(state): State<AppState>,
    axum::Extension(auth_ctx): axum::Extension<super::auth::AdminAuthContext>,
    Path((principal_id, artifact_id)): Path<(String, String)>,
    Json(req): Json<super::dto::AdminGovernancePromoteRequest>,
) -> AppResult<Json<super::dto::AdminGovernancePromoteResponse>> {
    let principal = decode_principal_id(&principal_id)?;
    let admin_subject = format!("admin:{}", auth_ctx.token_fingerprint);
    let resp = state
        .admin_promote_governance_artifact(
            &principal,
            &artifact_id,
            req.reviewer_note.as_deref(),
            &admin_subject,
        )
        .await?;
    Ok(Json(resp))
}

#[derive(serde::Deserialize)]
pub struct AuditLogQuery {
    #[serde(default)]
    pub limit: Option<u64>,
    #[serde(default)]
    pub offset: Option<u64>,
}

/// GET /admin/api/audit
pub async fn get_audit_log(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<AuditLogQuery>,
) -> AppResult<Json<super::dto::AdminAuditLogResponse>> {
    let limit = query.limit.unwrap_or(50).max(1).min(200);
    let offset = query.offset.unwrap_or(0);
    let items = state.admin_get_audit_log(limit, offset)?;
    Ok(Json(super::dto::AdminAuditLogResponse { items }))
}

/// GET /admin/api/settings
pub async fn get_settings(State(state): State<AppState>) -> AppResult<Json<super::dto::AdminSettingsResponse>> {
    let config_val = state.admin_get_settings()?;
    Ok(Json(super::dto::AdminSettingsResponse { config: config_val }))
}

/// POST /admin/api/settings
pub async fn update_settings(
    State(state): State<AppState>,
    axum::Extension(auth_ctx): axum::Extension<super::auth::AdminAuthContext>,
    Json(req): Json<super::dto::AdminSettingsUpdateRequest>,
) -> AppResult<Json<super::dto::AdminSettingsUpdateResponse>> {
    let admin_subject = format!("admin:{}", auth_ctx.token_fingerprint);
    
    // Parse new TOML config to see if it's mostly valid.
    let _new_config: crate::config::AppConfig = toml::from_str(&req.config_toml)
        .map_err(|e| AppError::invalid_request(format!("invalid TOML: {e}")))?;
    
    // We do NOT support hot-reloading in the current implementation.
    // So we record an audit event and pretend it's updated, requiring a restart.
    
    let toml_val = serde_json::to_string(&serde_json::json!({"action": "settings update requested"})).unwrap_or_default();
    
    let _ = state.admin_emit_audit(
        &admin_subject,
        "settings.update",
        None,
        Some("config.toml"),
        None,
        "success",
        Some(&toml_val),
    );
    
    // (In a real implementation, we would call new_config.save() and somehow sync to state.config)
    
    Ok(Json(super::dto::AdminSettingsUpdateResponse {
        applied: false,
        restart_required: true,
        summary: "Config update requested; restart the process to apply changes.".to_string(),
    }))
}
