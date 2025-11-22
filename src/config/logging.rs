use std::env;

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub channel: String,
    pub level: String,
    pub dir: String,
    pub file: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            channel: env::var("LOG_CHANNEL").unwrap_or_else(|_| "stack".to_string()),
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            dir: env::var("LOG_DIR").unwrap_or_else(|_| "storage/logs".to_string()),
            file: env::var("LOG_FILE").unwrap_or_else(|_| "webrust.log".to_string()),
        }
    }
}
