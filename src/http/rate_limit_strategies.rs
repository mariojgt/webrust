/// Strategic Rate Limiting Builders
/// Ready-to-use rate limiting strategies for different scenarios

use super::rate_limiter::{RateLimiter, RateLimitConfig};

/// Authentication attempt limiter (strict)
pub fn auth_limiter() -> RateLimiter {
    RateLimiter::new(
        RateLimitConfig::new(5, 900) // 5 attempts per 15 minutes
            .with_per_ip(true)
            .with_exclude_paths(vec!["/health".to_string()])
    )
}

/// API endpoint limiter (moderate)
pub fn api_limiter() -> RateLimiter {
    RateLimiter::new(
        RateLimitConfig::new(100, 60) // 100 requests per minute
            .with_per_ip(true)
            .with_exclude_paths(vec![
                "/health".to_string(),
                "/metrics".to_string(),
            ])
    )
}

/// Global rate limiter (lenient)
pub fn global_limiter() -> RateLimiter {
    RateLimiter::new(
        RateLimitConfig::new(1000, 3600) // 1000 requests per hour
            .with_per_ip(true)
            .with_exclude_paths(vec![
                "/health".to_string(),
                "/metrics".to_string(),
                "/status".to_string(),
            ])
    )
}

/// Sensitive endpoint limiter (very strict)
pub fn sensitive_limiter() -> RateLimiter {
    RateLimiter::new(
        RateLimitConfig::new(10, 3600) // 10 requests per hour
            .with_per_ip(true)
            .with_exclude_paths(vec!["/health".to_string()])
    )
}

/// Search endpoint limiter (moderate search protection)
pub fn search_limiter() -> RateLimiter {
    RateLimiter::new(
        RateLimitConfig::new(30, 60) // 30 searches per minute
            .with_per_ip(true)
    )
}

/// File upload limiter (strict for uploads)
pub fn upload_limiter() -> RateLimiter {
    RateLimiter::new(
        RateLimitConfig::new(20, 3600) // 20 uploads per hour
            .with_per_ip(true)
    )
}

/// Custom limiter builder
pub struct RateLimiterBuilder {
    config: RateLimitConfig,
}

impl RateLimiterBuilder {
    pub fn new() -> Self {
        Self {
            config: RateLimitConfig::default(),
        }
    }

    pub fn requests(mut self, max: u32) -> Self {
        self.config.max_requests = max;
        self
    }

    pub fn per_minute(self) -> Self {
        self.window(60)
    }

    pub fn per_hour(self) -> Self {
        self.window(3600)
    }

    pub fn per_day(self) -> Self {
        self.window(86400)
    }

    pub fn window(mut self, seconds: u64) -> Self {
        self.config.window_seconds = seconds;
        self
    }

    pub fn per_ip(mut self) -> Self {
        self.config.per_ip = true;
        self
    }

    pub fn per_user(mut self) -> Self {
        self.config.per_user = true;
        self
    }

    pub fn exclude_path(mut self, path: impl Into<String>) -> Self {
        self.config.exclude_paths.push(path.into());
        self
    }

    pub fn exclude_paths(mut self, paths: Vec<String>) -> Self {
        self.config.exclude_paths.extend(paths);
        self
    }

    pub fn build(self) -> RateLimiter {
        RateLimiter::new(self.config)
    }
}

impl Default for RateLimiterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_limiter() {
        let limiter = auth_limiter();
        assert_eq!(limiter.max_requests(), 5);
        assert_eq!(limiter.window_seconds(), 900);
    }

    #[test]
    fn test_api_limiter() {
        let limiter = api_limiter();
        assert_eq!(limiter.max_requests(), 100);
        assert_eq!(limiter.window_seconds(), 60);
    }

    #[test]
    fn test_global_limiter() {
        let limiter = global_limiter();
        assert_eq!(limiter.max_requests(), 1000);
        assert_eq!(limiter.window_seconds(), 3600);
    }

    #[test]
    fn test_sensitive_limiter() {
        let limiter = sensitive_limiter();
        assert_eq!(limiter.max_requests(), 10);
        assert_eq!(limiter.window_seconds(), 3600);
    }

    #[test]
    fn test_builder_requests() {
        let limiter = RateLimiterBuilder::new()
            .requests(50)
            .per_minute()
            .build();

        assert_eq!(limiter.max_requests(), 50);
        assert_eq!(limiter.window_seconds(), 60);
    }

    #[test]
    fn test_builder_per_hour() {
        let limiter = RateLimiterBuilder::new()
            .requests(100)
            .per_hour()
            .build();

        assert_eq!(limiter.window_seconds(), 3600);
    }

    #[test]
    fn test_builder_per_day() {
        let limiter = RateLimiterBuilder::new()
            .requests(1000)
            .per_day()
            .build();

        assert_eq!(limiter.window_seconds(), 86400);
    }

    #[test]
    fn test_builder_exclude_path() {
        let limiter = RateLimiterBuilder::new()
            .exclude_path("/health")
            .exclude_path("/status")
            .build();

        assert_eq!(limiter.config.exclude_paths.len(), 2);
    }

    #[test]
    fn test_builder_chaining() {
        let limiter = RateLimiterBuilder::new()
            .requests(200)
            .per_minute()
            .per_ip()
            .exclude_paths(vec!["/health".to_string()])
            .build();

        assert_eq!(limiter.max_requests(), 200);
        assert_eq!(limiter.window_seconds(), 60);
    }
}
