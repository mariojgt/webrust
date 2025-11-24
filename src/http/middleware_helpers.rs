/// Middleware helpers and utilities (Laravel-inspired)
/// Provides decorators and macros for easier middleware implementation

use axum::{
    middleware::Next,
    http::Request,
    body::Body,
    response::Response,
};

/// Middleware trait for easier implementation
pub trait Middleware {
    async fn handle(&self, req: Request<Body>, next: Next) -> Response;
}

/// Macro to create a simple middleware
#[macro_export]
macro_rules! middleware {
    ($name:ident, $logic:expr) => {
        pub async fn $name(req: Request<Body>, next: Next) -> Response {
            $logic(req, next).await
        }
    };
}

/// Rate limiting middleware marker
pub struct RateLimitMiddleware {
    pub max_requests: u32,
    pub window_seconds: u64,
}

impl RateLimitMiddleware {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

/// CORS middleware options
pub struct CorsMiddleware {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

impl CorsMiddleware {
    pub fn permissive() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
            ],
            allowed_headers: vec!["*".to_string()],
        }
    }

    pub fn new(
        allowed_origins: Vec<String>,
        allowed_methods: Vec<String>,
        allowed_headers: Vec<String>,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
        }
    }
}

/// Authenticate middleware marker
pub struct AuthMiddleware;

/// Guest middleware marker
pub struct GuestMiddleware;

/// Throttle middleware for rate limiting
pub struct ThrottleMiddleware {
    pub requests: u32,
    pub minutes: u32,
}

impl ThrottleMiddleware {
    pub fn new(requests: u32, minutes: u32) -> Self {
        Self { requests, minutes }
    }
}
