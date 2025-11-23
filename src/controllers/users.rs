use axum::{extract::State, response::Html};
use tera::Context;

use crate::framework::AppState;
use crate::models::user::User;
use crate::orbit::Orbit;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    // Use the new Orbit ORM method which handles connection selection
    let users = User::all(&state.db_manager)
        .await
        .unwrap_or_else(|_| Vec::new());

    let mut ctx = Context::new();
    ctx.insert("title", "Users");
    ctx.insert("users", &users);

    let body = state
        .templates
        .render("users/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {err}"));

    Html(body)
}
