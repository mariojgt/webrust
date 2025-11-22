pub mod web;
pub mod api;

use axum::{Router, routing::any};
use tower_http::services::ServeDir;
use crate::framework::AppState;
use crate::http::middleware::log_request;
use crate::http::middleware::csrf_protection;
use crate::controllers::error::not_found;
use tower_http::catch_panic::CatchPanicLayer;
use crate::http::panic::handle_panic;
use tower_sessions::{SessionManagerLayer, MemoryStore, Expiry};
use tower_sessions::cookie::Key;
use tower_sessions::cookie::time::Duration;

/// Combine all route groups and apply global middleware
pub fn router(state: AppState) -> Router {
    let web_routes = web::web(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), csrf_protection));

    let api_routes = api::api(state.clone());

    // Serve static files from "public" directory
    let static_files = ServeDir::new("public");

    // Session Config
    let store = MemoryStore::default();
    // In production, load this from .env
    let secret = "0123456789012345678901234567890123456789012345678901234567890123";
    let key = Key::from(secret.as_bytes());

    let session_layer = SessionManagerLayer::new(store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600)))
        .with_signed(key);

    Router::new()
        .merge(web_routes)
        .merge(api_routes)
        .nest_service("/public", static_files)
        .fallback(any(not_found).with_state(state))
        .layer(session_layer)
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(axum::middleware::from_fn(log_request))
}
