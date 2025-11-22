use axum::{
    extract::State,
    http::{header, Request},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde_json::json;
use tera::Context;

use crate::framework::AppState;

/// Handle 404 errors - returns JSON or HTML depending on request Accept header
pub async fn not_found(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> Response {
    let is_json = req
        .headers()
        .get(header::ACCEPT)
        .and_then(|h| h.to_str().ok())
        .map(|h| h.contains("application/json"))
        .unwrap_or(false);

    if is_json {
        // Return JSON 404
        (
            axum::http::StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Not Found",
                "message": "The requested resource was not found",
                "status": 404
            })),
        )
            .into_response()
    } else {
        // Return HTML 404 page
        let mut ctx = Context::new();
        ctx.insert("title", "404 - Page Not Found");

        match state.templates.render("errors/404.rune.html", &ctx) {
            Ok(body) => (axum::http::StatusCode::NOT_FOUND, Html(body)).into_response(),
            Err(_) => {
                // Fallback if template doesn't exist
                let fallback_html = r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title>404 - Not Found</title>
                        <style>
                            body { font-family: sans-serif; text-align: center; margin-top: 50px; }
                            h1 { font-size: 48px; color: #333; }
                            p { font-size: 18px; color: #666; }
                        </style>
                    </head>
                    <body>
                        <h1>404</h1>
                        <p>Page not found</p>
                    </body>
                    </html>
                "#;
                (axum::http::StatusCode::NOT_FOUND, Html(fallback_html.to_string())).into_response()
            }
        }
    }
}
