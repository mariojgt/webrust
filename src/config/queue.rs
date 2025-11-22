use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct QueueConfig {
    pub driver: String, // redis, sync
    pub redis_url: String,
    pub queue_name: String,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            driver: "sync".to_string(),
            redis_url: "redis://127.0.0.1:6379/".to_string(),
            queue_name: "default".to_string(),
        }
    }
}
