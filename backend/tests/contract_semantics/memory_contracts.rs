
use super::*;

#[tokio::test]
async fn store_supports_only_two_modes_and_preserves_tool_values() {
    let app = setup_app();

    let tool_store = json!({
        "actor": actor("u1", "main", "sess-1", "session-key-1"),
        "mode": "tool-store",
        "memory": {
            "text": "User prefers Neovim",
            "category": "preference",
            "importance": 0.82
        }
    });

    let (status, body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(tool_store),
        Some("idem-store-1"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["results"][0]["category"], "preference");
    assert_eq!(body["results"][0]["importance"], 0.82);

    let auto_capture = json!({
        "actor": actor("u1", "main", "sess-2", "session-key-1"),
        "mode": "auto-capture",
        "items": [
            { "role": "user", "text": "I use tmux" }
        ]
    });

    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(auto_capture),
        Some("idem-store-2"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let invalid_mode = json!({
        "actor": actor("u1", "main", "sess-3", "session-key-1"),
        "mode": "manual",
        "memory": { "text": "x" }
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(invalid_mode),
        Some("idem-store-3"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let auto_with_forbidden_fields = json!({
        "actor": actor("u1", "main", "sess-4", "session-key-1"),
        "mode": "auto-capture",
        "category": "preference",
        "items": [
            { "role": "user", "text": "hello" }
        ]
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(auto_with_forbidden_fields),
        Some("idem-store-4"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn write_payloads_forbid_scope_fields() {
    let app = setup_app();

    let tool_store_with_scope = json!({
        "actor": actor("u1", "main", "sess-1", "session-key-1"),
        "mode": "tool-store",
        "memory": {
            "text": "User prefers fish",
            "scope": "agent:evil"
        }
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(tool_store_with_scope),
        Some("idem-store-scope"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let valid_store = json!({
        "actor": actor("u1", "main", "sess-2", "session-key-1"),
        "mode": "tool-store",
        "memory": {
            "text": "User prefers zsh",
            "category": "preference"
        }
    });

    let (_, store_body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(valid_store),
        Some("idem-store-ok"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    let memory_id = store_body["results"][0]["id"]
        .as_str()
        .expect("memory id should be present");

    let update_with_scope = json!({
        "actor": actor("u1", "main", "sess-3", "session-key-1"),
        "memoryId": memory_id,
        "patch": {
            "scope": "agent:forbidden"
        }
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/update",
        Some(update_with_scope),
        Some("idem-update-scope"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn update_endpoint_exists_and_uses_backend_scope_derivation() {
    let app = setup_app();

    let store = json!({
        "actor": actor("u1", "main", "sess-1", "session-key-1"),
        "mode": "tool-store",
        "memory": {
            "text": "User prefers tmux",
            "category": "preference",
            "importance": 0.4
        }
    });
    let (_, store_body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(store),
        Some("idem-update-base-store"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    let memory_id = store_body["results"][0]["id"]
        .as_str()
        .expect("memory id should exist");

    let update = json!({
        "actor": actor("u1", "main", "sess-2", "session-key-1"),
        "memoryId": memory_id,
        "patch": {
            "text": "User prefers tmux with vim bindings",
            "importance": 0.9
        }
    });
    let (status, body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/update",
        Some(update),
        Some("idem-update-1"),
        Some(("u1", "main")),
        &[],
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["result"]["action"], "UPDATE");
    assert_eq!(body["result"]["scope"], "agent:main");
    assert_eq!(body["result"]["importance"], 0.9);
}

#[tokio::test]
async fn stats_route_is_post_only_and_session_id_is_ephemeral() {
    let app = setup_app();

    let store = json!({
        "actor": actor("u1", "main", "sess-original", "session-key-stable"),
        "mode": "tool-store",
        "memory": {
            "text": "Persist beyond runtime session"
        }
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(store),
        Some("idem-stats-store"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let stats_payload = json!({
        "actor": actor("u1", "main", "sess-new-runtime", "session-key-stable")
    });
    let (status, body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/stats",
        Some(stats_payload),
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["memoryCount"], 1);

    let (status, _) = request_json(
        &app,
        Method::GET,
        "/v1/memories/stats",
        None,
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn stats_actor_envelope_must_match_authenticated_principal() {
    let app = setup_app();

    let store = json!({
        "actor": actor("u1", "main", "sess-stats-store", "session-key-stats"),
        "mode": "tool-store",
        "memory": {
            "text": "stats envelope verification"
        }
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(store),
        Some("idem-stats-envelope-store"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let mismatched_stats = json!({
        "actor": actor("u1", "main", "sess-stats-mismatch", "session-key-stats")
    });
    let (status, body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/stats",
        Some(mismatched_stats),
        None,
        Some(("u2", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

#[tokio::test]
async fn list_default_order_and_final_page_next_offset_null() {
    let app = setup_app();

    for (idx, text) in ["first", "second", "third"].into_iter().enumerate() {
        let session_id = format!("sess-{idx}");
        let idem_key = format!("idem-list-{idx}");
        let store = json!({
            "actor": actor("u1", "main", &session_id, "session-key-1"),
            "mode": "tool-store",
            "memory": {
                "text": text,
                "category": "fact"
            }
        });
        let (status, _) = request_json(
            &app,
            Method::POST,
            "/v1/memories/store",
            Some(store),
            Some(&idem_key),
            Some(("u1", "main")),
            &[],
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        tokio::time::sleep(Duration::from_millis(2)).await;
    }

    let first_page = json!({
        "actor": actor("u1", "main", "sess-list-1", "session-key-1"),
        "limit": 2,
        "offset": 0,
        "category": "fact"
    });
    let (status, body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/list",
        Some(first_page),
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["rows"][0]["text"], "third");
    assert_eq!(body["nextOffset"], 2);

    let last_page = json!({
        "actor": actor("u1", "main", "sess-list-2", "session-key-1"),
        "limit": 2,
        "offset": 2,
        "category": "fact"
    });
    let (status, body) = request_json(
        &app,
        Method::POST,
        "/v1/memories/list",
        Some(last_page),
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["nextOffset"].is_null());
}

#[tokio::test]
async fn frozen_category_enum_is_enforced() {
    let app = setup_app();

    let store_invalid_category = json!({
        "actor": actor("u1", "main", "sess-1", "session-key-1"),
        "mode": "tool-store",
        "memory": {
            "text": "x",
            "category": "not-a-category"
        }
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/store",
        Some(store_invalid_category),
        Some("idem-bad-category"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let list_invalid_category = json!({
        "actor": actor("u1", "main", "sess-2", "session-key-1"),
        "limit": 10,
        "offset": 0,
        "category": "not-a-category"
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/memories/list",
        Some(list_invalid_category),
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn reflection_generation_routes_are_removed() {
    let app = setup_app();

    let enqueue = json!({
        "actor": actor("u1", "main", "sess-1", "session-key-1"),
        "trigger": "reset",
        "messages": [
            { "role": "user", "text": "summarize session" }
        ]
    });
    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/reflection/jobs",
        Some(enqueue),
        Some("idem-job-1"),
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, _) = request_json(
        &app,
        Method::POST,
        "/v1/reflection/source",
        Some(json!({
            "actor": actor("u1", "main", "sess-1", "session-key-1"),
            "trigger": "new",
            "maxMessages": 10
        })),
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, _) = request_json(
        &app,
        Method::GET,
        "/v1/reflection/jobs/job_removed",
        None,
        None,
        Some(("u1", "main")),
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
