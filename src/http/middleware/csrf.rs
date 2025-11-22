use axum::{
    body::Body,
    http::{Request, StatusCode, Method, request::Parts},
    middleware::Next,
    response::Response,
    extract::{FromRequestParts, State},
};
use tower_sessions::Session;
use rand::{Rng, distributions::Alphanumeric};
use async_trait::async_trait;
use crate::framework::AppState;

pub const CSRF_TOKEN_KEY: &str = "_csrf_token";
pub const CSRF_HEADER: &str = "X-CSRF-TOKEN";

pub struct CsrfToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for CsrfToken
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let token = session.get(CSRF_TOKEN_KEY).await.unwrap_or(None).unwrap_or_default();
        Ok(CsrfToken(token))
    }
}

pub async fn csrf_protection(
    State(state): State<AppState>,
    session: Session,
    req: Request<Body>,
    next: Next
) -> Result<Response, StatusCode> {
    // 0. Check for exclusions from Config
    let path = req.uri().path();
    for excluded in &state.config.csrf.except {
        let pattern = excluded.trim_end_matches('*');
        if path.starts_with(pattern) {
            return Ok(next.run(req).await);
        }
    }

    // 1. Ensure a token exists in the session
    // We use a block to drop the borrow of session
    let token: String = {
        match session.get(CSRF_TOKEN_KEY).await {
            Ok(Some(t)) => t,
            _ => {
                let t: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect();

                if let Err(_) = session.insert(CSRF_TOKEN_KEY, &t).await {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                t
            }
        }
    };

    // 2. Verify token on unsafe methods
    let method = req.method();
    if method == Method::POST || method == Method::PUT || method == Method::PATCH || method == Method::DELETE {
        let header_token = req.headers()
            .get(CSRF_HEADER)
            .and_then(|h| h.to_str().ok());

        if header_token != Some(&token) {
             return Err(StatusCode::FORBIDDEN);
        }
    }

    Ok(next.run(req).await)
}
