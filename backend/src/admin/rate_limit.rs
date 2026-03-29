use crate::error::{AppError, AppResult};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// In-memory per-IP+token-fingerprint rate limiter for the admin plane.
///
/// Uses a sliding window counter approach.
/// Independent from any runtime-plane rate-limiting.
#[derive(Clone)]
pub struct AdminRateLimiter {
    inner: Arc<Mutex<RateLimiterState>>,
    max_requests: u32,
    window_secs: u64,
}

struct RateLimiterState {
    buckets: HashMap<String, SlidingWindowBucket>,
}

struct SlidingWindowBucket {
    timestamps: Vec<Instant>,
}

impl AdminRateLimiter {
    /// Create a new admin rate limiter.
    ///
    /// `max_requests`: maximum requests allowed per window per identity.
    /// `window_secs`: sliding window duration in seconds.
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RateLimiterState {
                buckets: HashMap::new(),
            })),
            max_requests,
            window_secs,
        }
    }

    /// Check rate limit for a given remote_ip + token_fingerprint combination.
    ///
    /// Returns `Ok(())` if the request is allowed, or an admin-plane JSON 429
    /// error if rate-limited.
    pub fn check_rate_limit(
        &self,
        remote_ip: &str,
        token_fingerprint: &str,
    ) -> AppResult<()> {
        let key = format!("{}:{}", remote_ip, token_fingerprint);
        let now = Instant::now();
        let window = std::time::Duration::from_secs(self.window_secs);

        let mut state = self.inner.lock();
        let bucket = state
            .buckets
            .entry(key)
            .or_insert_with(|| SlidingWindowBucket {
                timestamps: Vec::new(),
            });

        // Evict expired entries.
        bucket.timestamps.retain(|ts| now.duration_since(*ts) < window);

        if bucket.timestamps.len() >= self.max_requests as usize {
            return Err(AppError::rate_limited(
                "admin API rate limit exceeded; try again later",
            ));
        }

        bucket.timestamps.push(now);
        Ok(())
    }
}
