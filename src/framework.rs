use std::env;
use std::sync::Arc;

use crate::database::{DbPool, DbPoolOptions};
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
    pub templates: Arc<Tera>,
    pub config: Arc<Config>,
    pub cache: Cache,
}

impl AppState {
    pub fn new(db: Option<DbPool>, templates: Tera, cache: Cache) -> Self {
        Self {
            db,
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

pub async fn build_pool() -> Result<DbPool, sqlx::Error> {
    let config = Config::new();

    DbPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
}
