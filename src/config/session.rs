use std::env;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub driver: String,
    pub lifetime: i64,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            driver: env::var("SESSION_DRIVER").unwrap_or_else(|_| "memory".to_string()),
            lifetime: env::var("SESSION_LIFETIME").unwrap_or_else(|_| "120".to_string()).parse().unwrap_or(120),
        }
    }
}
