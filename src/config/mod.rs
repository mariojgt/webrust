pub mod app;
pub mod csrf;
pub mod mail;
pub mod queue;
pub mod database;
pub mod session;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: app::AppConfig,
    pub csrf: csrf::CsrfConfig,
    pub mail: mail::MailConfig,
    pub queue: queue::QueueConfig,
    pub database: database::DatabaseConfig,
    pub session: session::SessionConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app: app::AppConfig::default(),
            csrf: csrf::CsrfConfig::default(),
            mail: mail::MailConfig::default(),
            queue: queue::QueueConfig::default(),
            database: database::DatabaseConfig::default(),
            session: session::SessionConfig::default(),
        }
    }
}
