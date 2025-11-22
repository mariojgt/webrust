use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CsrfConfig {
    pub except: Vec<String>,
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            except: vec![
                "/api/*".to_string(),
                "/webhooks/*".to_string(),
            ],
        }
    }
}
