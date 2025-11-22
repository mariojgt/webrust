pub mod web;
pub mod api;

use axum::{Router, routing::any};
use tower_http::services::ServeDir;
use crate::framework::AppState;
use crate::http::middleware::log_request;
use crate::controllers::error::not_found;
use tower_http::catch_panic::CatchPanicLayer;
use crate::http::panic::handle_panic;

/// Combine all route groups and apply global middleware
pub fn router(state: AppState) -> Router {
    let web_routes = web::web(state.clone());
    let api_routes = api::api(state.clone());

    // Serve static files from "public" directory
    let static_files = ServeDir::new("public");

    Router::new()
        .merge(web_routes)
        .merge(api_routes)
        .nest_service("/public", static_files)
        .fallback(any(not_found).with_state(state))
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(axum::middleware::from_fn(log_request))
}
