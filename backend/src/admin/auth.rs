use crate::error::{AppError, AppResult};
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request},
    middleware::Next,
    response::Response,
};

use super::rate_limit::AdminRateLimiter;

/// Extracted admin auth context attached to each authenticated admin request.
#[derive(Clone, Debug)]
pub struct AdminAuthContext {
    /// SHA-256 fingerprint (hex, first 16 chars) of the admin token.
    pub token_fingerprint: String,
}

/// Admin auth middleware: validates `Authorization: Bearer <admin_token>`,
/// rejects runtime tokens, and enforces admin rate-limiting.
pub async fn admin_auth_middleware(
    State((admin_token, runtime_token, rate_limiter)): State<(
        String,
        String,
        AdminRateLimiter,
    )>,
    request: Request<Body>,
    next: Next,
) -> AppResult<Response> {
    let token = admin_bearer_token(request.headers())?;

    // Reject runtime token on admin routes.
    if token == runtime_token {
        return Err(AppError::forbidden(
            "runtime bearer token is not accepted on admin API routes",
        ));
    }

    if token != admin_token {
        return Err(AppError::unauthorized("invalid admin bearer token"));
    }

    // Rate-limit check.
    let remote_ip = extract_remote_ip(&request);
    let token_fingerprint = token_fingerprint(&token);
    rate_limiter.check_rate_limit(&remote_ip, &token_fingerprint)?;

    let auth_ctx = AdminAuthContext { token_fingerprint };
    let mut request = request;
    request.extensions_mut().insert(auth_ctx);

    Ok(next.run(request).await)
}

fn admin_bearer_token(headers: &HeaderMap) -> AppResult<String> {
    let value = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::unauthorized("missing Authorization header on admin route"))?
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

fn extract_remote_ip(request: &Request<Body>) -> String {
    // Try X-Forwarded-For first, then fall back to peer address.
    if let Some(xff) = request.headers().get("x-forwarded-for") {
        if let Ok(val) = xff.to_str() {
            if let Some(first) = val.split(',').next() {
                let ip = first.trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }
    // Axum's ConnectInfo is not always available in test/oneshot mode;
    // fall back to "unknown".
    "unknown".to_string()
}

fn token_fingerprint(token: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
