pub mod web;
pub mod api;

use axum::{Router, routing::any};
use tower_http::services::ServeDir;
use crate::framework::AppState;
use crate::http::middleware::log_request;
use crate::http::middleware::csrf_protection;
use crate::http::middleware::inertia::share_inertia_data;
use crate::controllers::error::not_found;
use tower_http::catch_panic::CatchPanicLayer;
use crate::http::panic::handle_panic;
use tower_sessions::{SessionManagerLayer, MemoryStore, Expiry};
use tower_sessions::cookie::Key;
use tower_sessions::cookie::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::compression::CompressionLayer;

#[cfg(feature = "mysql")]
use tower_sessions_sqlx_store::MySqlStore as SqlxStore;
#[cfg(feature = "postgres")]
use tower_sessions_sqlx_store::PostgresStore as SqlxStore;
#[cfg(feature = "sqlite")]
use tower_sessions_sqlx_store::SqliteStore as SqlxStore;

/// Combine all route groups and apply global middleware
pub async fn router(state: AppState) -> Router {
    let web_routes = web::web(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), share_inertia_data))
        .layer(axum::middleware::from_fn_with_state(state.clone(), csrf_protection));

    let api_routes = api::api(state.clone())
        .layer(CorsLayer::permissive()); // Allow all origins for API

    // Serve static files from "public" directory
    let static_files = ServeDir::new("public");
    let build_files = ServeDir::new("public/build");

    // Session Config
    let key = Key::from(state.config.app.key.as_bytes());
    let lifetime = state.config.session.lifetime;

    let mut app = Router::new()
        .merge(web_routes)
        .merge(api_routes)
        .nest_service("/public", static_files)
        .nest_service("/build", build_files)
        .fallback(any(not_found).with_state(state.clone()))
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(axum::middleware::from_fn(log_request))
        .layer(CompressionLayer::new()); // Global compression

    #[cfg(debug_assertions)]
    {
        app = app.layer(tower_livereload::LiveReloadLayer::new());
    }

    match state.config.session.driver.as_str() {
        "database" => {
            if let Some(pool) = &state.db {
                let store = SqlxStore::new(pool.clone());
                store.migrate().await.expect("Failed to migrate session store");

                let session_layer = SessionManagerLayer::new(store)
                    .with_secure(false) // Set to true in production with HTTPS
                    .with_expiry(Expiry::OnInactivity(Duration::seconds(lifetime)))
                    .with_signed(key);

                app.layer(session_layer)
            } else {
                tracing::warn!("⚠️ Database session driver selected but no database connection available. Falling back to memory.");
                let store = MemoryStore::default();
                let session_layer = SessionManagerLayer::new(store)
                    .with_secure(false)
                    .with_expiry(Expiry::OnInactivity(Duration::seconds(lifetime)))
                    .with_signed(key);
                app.layer(session_layer)
            }
        },
        _ => {
            let store = MemoryStore::default();
            let session_layer = SessionManagerLayer::new(store)
                .with_secure(false)
                .with_expiry(Expiry::OnInactivity(Duration::seconds(lifetime)))
                .with_signed(key);
            app.layer(session_layer)
        }
    }
}
