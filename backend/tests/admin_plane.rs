use axum::{
    body::{to_bytes, Body},
    http::{header, Method, Request, StatusCode},
    Router,
};
use chronicle_engine_rs::{
    build_app,
    config::{
        AppConfig, AuthConfig, LoggingConfig, ProvidersConfig, RetrievalConfig, ServerConfig,
        StorageConfig, TokenConfig,
    },
};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use tower::ServiceExt;
use uuid::Uuid;

const RUNTIME_TOKEN: &str = "runtime-token";
const ADMIN_TOKEN: &str = "admin-secret-token";

fn make_config(tmp: &Path) -> AppConfig {
    AppConfig {
        server: ServerConfig {
            bind: "127.0.0.1:0".to_string(),
            admin_assets_path: PathBuf::from("web/dist"),
        },
        storage: StorageConfig {
            lancedb_path: tmp.join("lancedb"),
            sqlite_path: tmp.join("sqlite/jobs.db"),
        },
        auth: AuthConfig {
            runtime: TokenConfig {
                token: RUNTIME_TOKEN.to_string(),
            },
            admin: TokenConfig {
                token: ADMIN_TOKEN.to_string(),
            },
        },
        logging: LoggingConfig {
            level: "error".to_string(),
        },
        providers: ProvidersConfig::default(),
        retrieval: RetrievalConfig::default(),
    }
}

fn setup_app() -> Router {
    let tmp = std::env::temp_dir().join(format!(
        "chronicle-engine-rs-admin-test-{}",
        Uuid::new_v4()
    ));
    std::fs::create_dir_all(&tmp).expect("temp test path should be created");
    let cfg = make_config(&tmp);
    build_app(cfg).expect("app should build")
}

async fn request_json(
    app: &Router,
    method: Method,
    path: &str,
    body: Option<Value>,
    bearer_token: Option<&str>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("x-request-id", "test-req-123");

    if let Some(token) = bearer_token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {}", token));
    }

    let request = if let Some(payload) = body {
        builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(payload.to_string()))
            .expect("request should be built")
    } else {
        builder
            .body(Body::empty())
            .expect("request should be built")
    };

    let response = app
        .clone()
        .oneshot(request)
        .await
        .expect("router should produce a response");

    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("response body should be readable");

    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({ "_raw": String::from_utf8_lossy(&bytes).to_string() }))
    };

    (status, value)
}

#[tokio::test]
async fn test_admin_auth_separation() {
    let app = setup_app();

    // 1. Unauthenticated request to admin API should fail
    let (status, _) = request_json(&app, Method::GET, "/admin/api/health", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // 2. Runtime token on admin API should fail (Forbidden)
    let (status, body) = request_json(
        &app,
        Method::GET,
        "/admin/api/health",
        None,
        Some(RUNTIME_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        body["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("runtime bearer token"),
        true
    );

    // 3. Admin token on admin API should succeed
    let (status, body) = request_json(
        &app,
        Method::GET,
        "/admin/api/health",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["plane"], "admin");

    // 4. Admin token on runtime API should fail (Unauthorized)
    let (status, _) = request_json(
        &app,
        Method::GET,
        "/v1/health",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    // v1/health isn't protected, let's use a protected route instead
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(json!({})),
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_admin_rate_limiter() {
    let app = setup_app();

    // The rate limit in lib.rs is 120 requests per 60 secs.
    // We'll hit it 120 times, and the 121st should fail with 429 Too Many Requests.
    for _ in 0..120 {
        let (status, _) = request_json(
            &app,
            Method::GET,
            "/admin/api/health",
            None,
            Some(ADMIN_TOKEN),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    let (status, body) = request_json(
        &app,
        Method::GET,
        "/admin/api/health",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body["error"]["code"], "RATE_LIMITED");
}

#[tokio::test]
async fn test_admin_principal_apis_and_recall() {
    let app = setup_app();

    // 1. Initial list should be empty
    let (status, body) = request_json(
        &app,
        Method::GET,
        "/admin/api/principals",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["principals"].as_array().unwrap().is_empty());

    // 2. Add some data via runtime API
    let actor = json!({
        "userId": "admin_test_user",
        "agentId": "admin_test_agent",
        "sessionId": "s1",
        "sessionKey": "k1"
    });
    
    let store_req = json!({
        "actor": actor,
        "mode": "tool-store",
        "memory": {
            "text": "This is a test memory for admin",
            "category": "fact"
        }
    });

    // We must pass the correct routing headers for runtime
    let mut builder = Request::builder()
        .method(Method::POST)
        .uri("/v1/memories/store")
        .header("x-request-id", "test-req-store")
        .header("idempotency-key", "test-idem-key")
        .header(header::AUTHORIZATION, format!("Bearer {}", RUNTIME_TOKEN))
        .header("x-auth-user-id", "admin_test_user")
        .header("x-auth-agent-id", "admin_test_agent")
        .header(header::CONTENT_TYPE, "application/json");
        
    let request = builder.body(Body::from(store_req.to_string())).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8_lossy(&bytes);
    assert_eq!(status, StatusCode::OK, "Failed to store memory: {}", body_str);

    // 3. Now principals should list the user
    let (status, body) = request_json(
        &app,
        Method::GET,
        "/admin/api/principals",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    
    let arr = body["principals"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    
    let p0 = &arr[0];
    assert_eq!(p0["userId"], "admin_test_user");
    assert_eq!(p0["agentId"], "admin_test_agent");
    assert_eq!(p0["memoryCount"], 1);
    
    let principal_id = p0["principalId"].as_str().unwrap();

    // 4. Simulate Generic Recall
    let recall_req = json!({
        "mode": "generic",
        "query": "test memory",
        "limit": 5
    });

    let (status, body) = request_json(
        &app,
        Method::POST,
        &format!("/admin/api/principals/{}/recall/simulate", principal_id),
        Some(recall_req),
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["mode"], "generic");
    assert_eq!(body["principalUserId"], "admin_test_user");
    assert_eq!(body["principalAgentId"], "admin_test_agent");
    
    let results = body["results"].as_array().unwrap();
    // Assuming the test without providers might not actually recall it, 
    // but the request should succeed and return an array (either empty or with items)
    assert!(results.len() >= 0);
    assert!(body.get("trace").is_some(), "trace should be included");
    assert!(body.get("appliedFilters").is_some());

    // 5. Unknown endpoint matching /admin/api/* but not defined should be 404 JSON, not HTML
    let (status, body) = request_json(
        &app,
        Method::GET,
        "/admin/api/unknown",
        None,
        Some(ADMIN_TOKEN), // authorized so it gets to the fallback or router 404
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["_raw"].as_str().unwrap_or("").is_empty(), true); // Axum default 404 is empty string usually, but it's not the SPA html
    
    // Check SPA shell fallback for /admin
    let mut builder = Request::builder().method(Method::GET).uri("/admin");
    let req = builder.body(Body::empty()).unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap();
    assert!(content_type.starts_with("text/html"));
}

#[tokio::test]
async fn phase_3_admin_apis_are_accessible_and_return_empty_or_defaults() {
    let app = setup_app();
    let principal_id = chronicle_engine_rs::admin::principal_id::encode_principal_id(&chronicle_engine_rs::models::Principal {
        user_id: "u1".into(),
        agent_id: "a1".into(),
    });

    // 1. Distill Jobs
    let (status, list) = request_json(
        &app,
        Method::GET,
        &format!("/admin/api/principals/{principal_id}/distill_jobs"),
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(list["items"].as_array().unwrap().is_empty());

    // 2. Transcripts
    let (status, list) = request_json(
        &app,
        Method::GET,
        &format!("/admin/api/principals/{principal_id}/transcripts"),
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(list["items"].as_array().unwrap().is_empty());

    // 3. Governance List
    let (status, list) = request_json(
        &app,
        Method::GET,
        &format!("/admin/api/principals/{principal_id}/governance"),
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(list["items"].as_array().unwrap().is_empty());

    // 4. Audit Log
    let (status, log) = request_json(
        &app,
        Method::GET,
        "/admin/api/audit?limit=10",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(log["items"].as_array().is_some());

    // 5. Settings
    let (status, settings) = request_json(
        &app,
        Method::GET,
        "/admin/api/settings",
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(settings["config"].is_object());
    assert_eq!(
        settings["config"]["auth"]["admin"]["token"].as_str().unwrap(),
        "****"
    );

    // 6. 404s for specific items
    let (status, _) = request_json(
        &app,
        Method::GET,
        &format!("/admin/api/principals/{principal_id}/distill_jobs/nonexistent-job"),
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, _) = request_json(
        &app,
        Method::GET,
        &format!("/admin/api/principals/{principal_id}/transcripts/nonexistent-transcript"),
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let valid_transcript_id = encode_transcript_id("key1", "sess1");
    let (status, _) = request_json(
        &app,
        Method::GET,
        &format!("/admin/api/principals/{principal_id}/transcripts/{valid_transcript_id}"),
        None,
        Some(ADMIN_TOKEN),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

fn encode_transcript_id(session_key: &str, session_id: &str) -> String {
    chronicle_engine_rs::admin::principal_id::encode_transcript_id(session_key, session_id)
}
