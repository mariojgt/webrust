use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use serde_json::json;
use tower_sessions::Session;
use crate::services::{auth::Auth, flash::Flash, validation::ValidationErrors};
use crate::http::inertia::SharedInertiaProps;

pub async fn share_inertia_data(
    session: Session,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let mut props = json!({});

    // Share Auth User
    if let Some(user_id) = Auth::id(&session).await {
        // In a real app, we might fetch the full user here
        props["auth"] = json!({ "user": { "id": user_id } });
    } else {
        props["auth"] = json!({ "user": null });
    }

    // Share Flash Messages
    let messages = Flash::get_all(&session).await;
    let mut flash = json!({});
    for msg in messages {
        flash[msg.kind] = json!(msg.message);
    }
    props["flash"] = flash;

    // Share Validation Errors
    let errors = ValidationErrors::get(&session).await;
    props["errors"] = json!(errors);

    req.extensions_mut().insert(SharedInertiaProps(props));

    next.run(req).await
}
