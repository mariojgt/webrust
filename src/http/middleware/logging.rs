use std::time::Instant;

use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn log_request(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    let status = response.status();
    let status_code = status.as_u16();

    // Color status based on code
    let status_emoji = match status_code {
        200..=299 => "✅",
        300..=399 => "↩️ ",
        400..=499 => "⚠️ ",
        500..=599 => "❌",
        _ => "❓",
    };

    let elapsed_ms = elapsed.as_millis();
    info!(
        "{} {} {} {} [{}ms]",
        status_emoji, method, uri, status_code, elapsed_ms
    );

    response
}
