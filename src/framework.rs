use std::env;
use std::sync::Arc;

use crate::database::{DbPool, DbPoolOptions, DatabaseManager};
use tera::Tera;
use crate::config::Config;
use crate::cache::Cache;
use axum::Router;

pub trait WebRustPackage {
    fn name(&self) -> &str;
    fn routes(&self, state: AppState) -> Router<AppState>;
}

#[derive(Clone)]
pub struct AppState {
    pub db: Option<DbPool>,
    pub db_manager: DatabaseManager,
    pub templates: Arc<Tera>,
    pub config: Arc<Config>,
    pub cache: Cache,
}

impl AppState {
    pub fn new(db_manager: DatabaseManager, templates: Tera, cache: Cache) -> Self {
        let db = db_manager.default_connection().cloned();
        Self {
            db,
            db_manager,
            templates: Arc::new(templates),
            config: Arc::new(Config::new()),
            cache,
        }
    }
}

pub fn build_tera() -> Result<Tera, tera::Error> {
    // Load *.rune.html as our Blade-like view templates
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec![".html", ".rune.html"]);
    Ok(tera)
}

pub async fn build_database_manager() -> DatabaseManager {
    let config = Config::new();
    let mut manager = DatabaseManager::new(config.database.default.clone());

    for (name, conn_config) in &config.database.connections {
        if conn_config.url.is_empty() {
            tracing::warn!("⚠️  Database connection '{}' has no URL configured. Skipping.", name);
            continue;
        }

        // We currently only support the compiled feature's driver (e.g. mysql)
        // So we ignore conn_config.driver for now, or warn if it doesn't match.
        // Ideally we would check if conn_config.driver matches the compiled feature.

        let pool_result = DbPoolOptions::new()
            .max_connections(conn_config.max_connections)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&conn_config.url)
            .await;

        match pool_result {
            Ok(pool) => {
                tracing::info!("✅ Database connection '{}' established", name);
                manager.add(name, pool);
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to connect to database '{}': {}. Skipping.", name, e);
            }
        }
    }

    manager
}
