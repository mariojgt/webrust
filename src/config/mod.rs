pub mod app;
pub mod csrf;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: app::AppConfig,
    pub csrf: csrf::CsrfConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app: app::AppConfig::default(),
            csrf: csrf::CsrfConfig::default(),
        }
    }
}
