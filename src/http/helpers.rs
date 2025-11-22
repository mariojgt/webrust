use axum::{
    response::{Html, IntoResponse, Response},
    http::StatusCode,
};
use tera::Context;
use crate::framework::AppState;

/// Render a view
/// Usage: view(&state, "home/index", &ctx)
pub fn view(state: &AppState, name: &str, ctx: &Context) -> Html<String> {
    // Append .rune.html if not present
    let template_name = if name.ends_with(".rune.html") || name.ends_with(".html") {
        name.to_string()
    } else {
        format!("{}.rune.html", name)
    };

    let body = state.templates.render(&template_name, ctx)
        .unwrap_or_else(|err| format!("View Error: {}", err));

    Html(body)
}

/// Abort with a status code
pub fn abort(code: u16) -> Response {
    let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, format!("Error {}", code)).into_response()
}

/// Return a JSON response (wrapper)
pub fn json<T: serde::Serialize>(data: T) -> axum::Json<T> {
    axum::Json(data)
}
