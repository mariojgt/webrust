/// Rate Limiting Middleware for Request Throttling
/// Provides protection against abuse and fair resource distribution

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    http::Request,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limit entry tracking requests
#[derive(Clone, Debug)]
struct RateLimitEntry {
    requests: Vec<Instant>,
    last_reset: Instant,
}

/// Rate limiting configuration
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Maximum requests allowed
    pub max_requests: u32,
    /// Time window in seconds
    pub window_seconds: u64,
    /// Enable per-IP limiting
    pub per_ip: bool,
    /// Enable per-user limiting
    pub per_user: bool,
    /// Exclude paths (exact match)
    pub exclude_paths: Vec<String>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window_seconds: 60,
            per_ip: true,
            per_user: false,
            exclude_paths: vec![
                "/health".to_string(),
                "/metrics".to_string(),
            ],
        }
    }
}

impl RateLimitConfig {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
            ..Default::default()
        }
    }

    pub fn with_per_user(mut self, per_user: bool) -> Self {
        self.per_user = per_user;
        self
    }

    pub fn with_per_ip(mut self, per_ip: bool) -> Self {
        self.per_ip = per_ip;
        self
    }

    pub fn with_exclude_paths(mut self, paths: Vec<String>) -> Self {
        self.exclude_paths = paths;
        self
    }
}

/// Global rate limit store
pub struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Check if request should be allowed
    pub async fn check(&self, key: &str) -> bool {
        let mut limits = self.limits.write().await;
        let window = Duration::from_secs(self.config.window_seconds);
        let now = Instant::now();

        let entry = limits
            .entry(key.to_string())
            .or_insert_with(|| RateLimitEntry {
                requests: Vec::new(),
                last_reset: now,
            });

        // Clean old requests outside the window
        entry.requests.retain(|&instant| now.duration_since(instant) < window);

        // Check if under limit
        if entry.requests.len() < self.config.max_requests as usize {
            entry.requests.push(now);
            true
        } else {
            false
        }
    }

    /// Get current request count for a key
    pub async fn get_count(&self, key: &str) -> u32 {
        let limits = self.limits.read().await;
        let window = Duration::from_secs(self.config.window_seconds);
        let now = Instant::now();

        if let Some(entry) = limits.get(key) {
            entry
                .requests
                .iter()
                .filter(|&&instant| now.duration_since(instant) < window)
                .count() as u32
        } else {
            0
        }
    }

    /// Get remaining requests for a key
    pub async fn get_remaining(&self, key: &str) -> u32 {
        let count = self.get_count(key).await;
        self.config.max_requests.saturating_sub(count)
    }

    /// Get window seconds
    pub fn window_seconds(&self) -> u64 {
        self.config.window_seconds
    }

    /// Get max requests
    pub fn max_requests(&self) -> u32 {
        self.config.max_requests
    }

    /// Reset limit for a key
    pub async fn reset(&self, key: &str) {
        self.limits.write().await.remove(key);
    }

    /// Clear all limits
    pub async fn clear(&self) {
        self.limits.write().await.clear();
    }

    /// Check if path is excluded
    fn is_excluded(&self, path: &str) -> bool {
        self.config.exclude_paths.iter().any(|p| p == path)
    }
}

/// Extract IP address from request
fn get_client_ip(headers: &axum::http::HeaderMap, addr: Option<ConnectInfo<SocketAddr>>) -> String {
    // Try X-Forwarded-For first (proxy)
    if let Some(header) = headers.get("x-forwarded-for") {
        if let Ok(s) = header.to_str() {
            return s.split(',').next().unwrap_or("unknown").trim().to_string();
        }
    }

    // Try X-Real-IP (proxy)
    if let Some(header) = headers.get("x-real-ip") {
        if let Ok(s) = header.to_str() {
            return s.to_string();
        }
    }

    // Use ConnectInfo
    addr.map(|info| info.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Rate limit response
pub struct RateLimitResponse {
    pub limited: bool,
    pub remaining: u32,
    pub retry_after: u64,
}

impl RateLimitResponse {
    pub fn to_response(&self) -> Response {
        let body = format!(
            r#"{{"error":"Too Many Requests","remaining":{},"retry_after":{}}}"#,
            self.remaining, self.retry_after
        );

        (
            StatusCode::TOO_MANY_REQUESTS,
            [("Retry-After", self.retry_after.to_string().as_str())],
            body,
        )
            .into_response()
    }
}

/// Middleware for rate limiting
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let limiter = Arc::new(RateLimiter::new(RateLimitConfig::default()));

    let ip = get_client_ip(req.headers(), Some(ConnectInfo(addr)));
    let path = req.uri().path();

    // Check if path is excluded
    if limiter.config.exclude_paths.iter().any(|p| p == path) {
        return next.run(req).await;
    }

    // Check rate limit
    if !limiter.check(&ip).await {
        let remaining = limiter.get_remaining(&ip).await;
        let retry_after = limiter.config.window_seconds;

        return RateLimitResponse {
            limited: true,
            remaining,
            retry_after,
        }
        .to_response();
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let limiter = RateLimiter::new(RateLimitConfig::default());
        assert_eq!(limiter.max_requests(), 60);
        assert_eq!(limiter.window_seconds(), 60);
    }

    #[tokio::test]
    async fn test_rate_limit_check() {
        let limiter = RateLimiter::new(RateLimitConfig::new(3, 1));
        
        assert!(limiter.check("user1").await);
        assert!(limiter.check("user1").await);
        assert!(limiter.check("user1").await);
        assert!(!limiter.check("user1").await);
    }

    #[tokio::test]
    async fn test_get_count() {
        let limiter = RateLimiter::new(RateLimitConfig::new(5, 1));
        
        limiter.check("user2").await;
        limiter.check("user2").await;
        
        assert_eq!(limiter.get_count("user2").await, 2);
    }

    #[tokio::test]
    async fn test_get_remaining() {
        let limiter = RateLimiter::new(RateLimitConfig::new(5, 1));
        
        limiter.check("user3").await;
        limiter.check("user3").await;
        
        assert_eq!(limiter.get_remaining("user3").await, 3);
    }

    #[tokio::test]
    async fn test_per_ip_isolation() {
        let limiter = RateLimiter::new(RateLimitConfig::new(2, 1));
        
        assert!(limiter.check("192.168.1.1").await);
        assert!(limiter.check("192.168.1.1").await);
        assert!(!limiter.check("192.168.1.1").await);
        
        // Different IP should not be limited
        assert!(limiter.check("192.168.1.2").await);
    }

    #[tokio::test]
    async fn test_reset() {
        let limiter = RateLimiter::new(RateLimitConfig::new(2, 1));
        
        limiter.check("user4").await;
        limiter.check("user4").await;
        assert!(!limiter.check("user4").await);
        
        limiter.reset("user4").await;
        assert!(limiter.check("user4").await);
    }

    #[tokio::test]
    async fn test_clear() {
        let limiter = RateLimiter::new(RateLimitConfig::new(2, 1));
        
        limiter.check("user5").await;
        limiter.check("user6").await;
        
        limiter.clear().await;
        
        assert_eq!(limiter.get_count("user5").await, 0);
        assert_eq!(limiter.get_count("user6").await, 0);
    }

    #[test]
    fn test_config_builder() {
        let config = RateLimitConfig::new(100, 30)
            .with_per_user(true)
            .with_per_ip(false)
            .with_exclude_paths(vec!["/health".to_string()]);
        
        assert_eq!(config.max_requests, 100);
        assert_eq!(config.window_seconds, 30);
        assert!(config.per_user);
        assert!(!config.per_ip);
        assert!(config.exclude_paths.contains(&"/health".to_string()));
    }

    #[test]
    fn test_default_config() {
        let config = RateLimitConfig::default();
        assert_eq!(config.max_requests, 60);
        assert_eq!(config.window_seconds, 60);
        assert!(config.per_ip);
    }
}
