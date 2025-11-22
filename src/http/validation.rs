use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
    Json, Form,
};
use serde::de::DeserializeOwned;
use validator::Validate;

/// A Laravel-like Form Request extractor.
/// It automatically deserializes the request body (JSON or Form)
/// and runs validation rules defined in the struct.
pub struct FormRequest<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for FormRequest<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let payload = if content_type.starts_with("application/json") {
            let Json(payload) = Json::<T>::from_request(req, state)
                .await
                .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())?;
            payload
        } else {
            // Default to Form for other content types or missing content type
            let Form(payload) = Form::<T>::from_request(req, state)
                .await
                .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())?;
            payload
        };

        if let Err(errors) = payload.validate() {
            // Return a 422 Unprocessable Entity with the validation errors
            // In a real app, you might want to redirect back with errors if it's a web request
            return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(errors)).into_response());
        }

        Ok(FormRequest(payload))
    }
}
