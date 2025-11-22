use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn api_auth(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Get token from .env
    let expected_token = env::var("API_TOKEN").unwrap_or_else(|_| "secret-token".to_string());

    // Check Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == format!("Bearer {}", expected_token) {
                return Ok(next.run(req).await);
            }
        }
    }

    // Also check query param ?token=... for easier browser testing
    if let Some(query) = req.uri().query() {
        if query.contains(&format!("token={}", expected_token)) {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
