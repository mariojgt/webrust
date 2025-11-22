use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MailConfig {
    pub driver: String, // smtp, log
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub from_address: String,
    pub from_name: String,
}

impl Default for MailConfig {
    fn default() -> Self {
        Self {
            driver: "log".to_string(),
            host: "127.0.0.1".to_string(),
            port: 1025,
            username: None,
            password: None,
            from_address: "hello@example.com".to_string(),
            from_name: "WebRust App".to_string(),
        }
    }
}
