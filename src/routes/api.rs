use axum::Router;
use crate::route;
use crate::framework::AppState;

/// API routes - for your JSON APIs
pub fn api(state: AppState) -> Router {
    route::api()
        // Add your API routes here
        // Example: .get("/users", api_users_index)
        .build()
        .with_state(state)
}
