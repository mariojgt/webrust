use axum::{extract::State, response::Html};
use tera::Context;

use crate::framework::AppState;
use crate::models::user::User;
use crate::orbit::Orbit;
use crate::http::error::AppError;
use crate::{dd, dump};

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    // Use the new Orbit ORM method which handles connection selection
    // User::query()
    //     .where_eq("email", "mariojgt2@gmail.com")
    //     .dd(); // Debug the SQL query

    // This code is unreachable because dd() panics
    // let user = User::query()
    //     .where_eq("email", "user@gmail.com")
    //     .first(&state.db_manager)
    // .await?;

    // dd!(user); // Debug the fetched user

    let users = User::all(&state.db_manager).await?;

    let mut ctx = Context::new();
    ctx.insert("title", "Users");
    ctx.insert("users", &users);

    let body = state
        .templates
        .render("users/index.rune.html", &ctx)
        .map_err(AppError::Template)?;

    Ok(Html(body))
}
