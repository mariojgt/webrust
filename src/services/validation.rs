use tower_sessions::Session;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub const VALIDATION_SESSION_KEY: &str = "_validation_errors";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ValidationErrors(pub HashMap<String, Vec<String>>);

impl ValidationErrors {
    pub async fn flash(session: &Session, errors: HashMap<String, Vec<String>>) {
        session.insert(VALIDATION_SESSION_KEY, errors).await.unwrap();
    }

    pub async fn get(session: &Session) -> HashMap<String, Vec<String>> {
        let errors: Option<HashMap<String, Vec<String>>> = session.get(VALIDATION_SESSION_KEY).await.unwrap();
        if errors.is_some() {
            session.remove::<HashMap<String, Vec<String>>>(VALIDATION_SESSION_KEY).await.unwrap();
        }
        errors.unwrap_or_default()
    }
}
