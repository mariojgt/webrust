use axum::{Router, middleware};
use crate::route;
use crate::framework::AppState;
use crate::http::middleware::api_auth;
use axum::response::Json;
use serde_json::json;

/// API routes - for your JSON APIs
pub fn api(state: AppState) -> Router {
    let protected_routes = route::api()
        .get("/user", || async {
            Json(json!({
                "id": 1,
                "name": "Admin User",
                "email": "admin@example.com"
            }))
        })
        .build()
        .layer(middleware::from_fn(api_auth))
        .with_state(state.clone());

    let public_routes = route::api()
        .get("/status", || async {
            Json(json!({ "status": "ok" }))
        })
        .build()
        .with_state(state);

    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
}
