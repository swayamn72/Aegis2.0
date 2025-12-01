use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    attempts: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_attempts: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_attempts: usize, window_minutes: u64) -> Self {
        Self {
            attempts: Arc::new(Mutex::new(HashMap::new())),
            max_attempts,
            window_duration: Duration::from_secs(window_minutes * 60),
        }
    }

    pub fn check_rate_limit(&self, key: &str) -> bool {
        let mut attempts = self.attempts.lock().unwrap();
        let now = Instant::now();

        let user_attempts = attempts.entry(key.to_string()).or_insert_with(Vec::new);

        // Remove old attempts outside the window
        user_attempts
            .retain(|&attempt_time| now.duration_since(attempt_time) < self.window_duration);

        if user_attempts.len() >= self.max_attempts {
            false
        } else {
            user_attempts.push(now);
            true
        }
    }
}

pub fn create_login_rate_limiter() -> RateLimiter {
    RateLimiter::new(5, 15) // 5 attempts per 15 minutes
}

pub async fn login_rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let limiter = create_login_rate_limiter();
    let ip = addr.ip().to_string();

    // Only apply rate limiting to auth endpoints
    if request.uri().path().contains("/auth/login")
        || request.uri().path().contains("/auth/register")
    {
        if !limiter.check_rate_limit(&ip) {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    Ok(next.run(request).await)
}
