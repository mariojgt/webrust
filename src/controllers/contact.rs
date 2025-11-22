use axum::{extract::State, Json};
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;

use crate::framework::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct ContactForm {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    #[validate(email(message = "Must be a valid email"))]
    pub email: String,

    #[validate(length(min = 10, message = "Message must be at least 10 characters"))]
    pub message: String,
}

pub async fn submit(
    State(_state): State<AppState>,
    Json(payload): Json<ContactForm>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        let mut error_map = serde_json::Map::new();
        for (field, errs) in errors.field_errors().iter() {
            let messages: Vec<String> = errs
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();
            error_map.insert(field.to_string(), json!(messages));
        }

        let body: Value = json!({
            "ok": false,
            "errors": error_map
        });

        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(body));
    }

    let body: Value = json!({
        "ok": true,
        "message": "Contact form validated successfully (here is where you would save to DB or send email)."
    });

    (axum::http::StatusCode::OK, Json(body))
}
