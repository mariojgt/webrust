use std::env;
use std::sync::Arc;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use tera::Tera;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: Option<MySqlPool>,
    pub templates: Arc<Tera>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: Option<MySqlPool>, templates: Tera) -> Self {
        Self {
            db,
            templates: Arc::new(templates),
            config: Arc::new(Config::new()),
        }
    }
}

pub fn build_tera() -> Result<Tera, tera::Error> {
    // Load *.rune.html as our Blade-like view templates
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec![".html", ".rune.html"]);
    Ok(tera)
}

pub async fn build_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://user:password@localhost:3306/webrust_app".to_string());

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
