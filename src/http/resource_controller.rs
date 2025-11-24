/// ResourceController trait for Laravel-like RESTful resource handling
/// Provides standard CRUD operations with minimal boilerplate
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use crate::framework::AppState;

/// Standard resource controller operations (like Laravel)
pub trait ResourceController: Send + Sync + 'static {
    /// GET /resources - List all resources
    async fn index(state: AppState) -> impl IntoResponse;

    /// GET /resources/create - Show creation form
    async fn create(state: AppState) -> impl IntoResponse {
        axum::response::Json(serde_json::json!({
            "message": "Create form not implemented"
        }))
    }

    /// POST /resources - Store a new resource
    async fn store(state: AppState) -> impl IntoResponse;

    /// GET /resources/:id - Show a single resource
    async fn show(state: AppState, id: i64) -> impl IntoResponse;

    /// GET /resources/:id/edit - Show edit form
    async fn edit(state: AppState, id: i64) -> impl IntoResponse {
        axum::response::Json(serde_json::json!({
            "message": "Edit form not implemented"
        }))
    }

    /// PUT/PATCH /resources/:id - Update a resource
    async fn update(state: AppState, id: i64) -> impl IntoResponse;

    /// DELETE /resources/:id - Delete a resource
    async fn destroy(state: AppState, id: i64) -> impl IntoResponse;
}

/// Simplified resource controller for API-only resources (no forms)
pub trait ApiResourceController: Send + Sync + 'static {
    /// GET /api/resources
    async fn index(state: AppState) -> impl IntoResponse;

    /// POST /api/resources
    async fn store(state: AppState) -> impl IntoResponse;

    /// GET /api/resources/:id
    async fn show(state: AppState, id: i64) -> impl IntoResponse;

    /// PUT/PATCH /api/resources/:id
    async fn update(state: AppState, id: i64) -> impl IntoResponse;

    /// DELETE /api/resources/:id
    async fn destroy(state: AppState, id: i64) -> impl IntoResponse;
}
