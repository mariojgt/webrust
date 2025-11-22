use tower_sessions::Session;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlashMessage {
    pub kind: String, // "success", "error", "info"
    pub message: String,
}

pub const FLASH_SESSION_KEY: &str = "_flash";

pub struct Flash;

impl Flash {
    pub async fn push(session: &Session, kind: &str, message: &str) {
        let mut messages: Vec<FlashMessage> = session.get(FLASH_SESSION_KEY).await.unwrap().unwrap_or_default();
        messages.push(FlashMessage {
            kind: kind.to_string(),
            message: message.to_string(),
        });
        session.insert(FLASH_SESSION_KEY, messages).await.unwrap();
    }

    pub async fn success(session: &Session, message: &str) {
        Self::push(session, "success", message).await;
    }

    pub async fn error(session: &Session, message: &str) {
        Self::push(session, "error", message).await;
    }
    
    // This should be called by middleware to inject into template context
    pub async fn get_all(session: &Session) -> Vec<FlashMessage> {
        let messages: Option<Vec<FlashMessage>> = session.get(FLASH_SESSION_KEY).await.unwrap();
        if messages.is_some() {
            session.remove::<Vec<FlashMessage>>(FLASH_SESSION_KEY).await.unwrap();
        }
        messages.unwrap_or_default()
    }
}
