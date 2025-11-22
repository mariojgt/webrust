use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub env: String,
    pub debug: bool,
    pub url: String,
    pub key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: std::env::var("APP_NAME").unwrap_or_else(|_| "WebRust".to_string()),
            env: std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string()),
            debug: std::env::var("APP_DEBUG").unwrap_or_else(|_| "true".to_string()) == "true",
            url: std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:8000".to_string()),
            key: std::env::var("APP_KEY").expect("APP_KEY must be set in .env file"),
        }
    }
}
