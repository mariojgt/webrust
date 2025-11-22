use crate::prelude::*;
use crate::requests::auth::{ResetPasswordWithTokenRequest, LoginRequest};
use axum::response::{IntoResponse, Redirect};

pub async fn login_form(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Login");
    
    // Pass flash messages to view
    let messages = Flash::get_all(&session).await;
    ctx.insert("flash_messages", &messages);

    let body = state.templates.render("auth/login.rune.html", &ctx).unwrap();
    Html(body)
}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    FormRequest(req): FormRequest<LoginRequest>
) -> impl IntoResponse {
    // In a real app, you'd check the database. 
    // For this demo, we'll mock a successful login if email is "admin@example.com" and password is "password"
    // OR if the database check passes (if you have the DB set up).
    
    let mut logged_in = false;

    if let Some(pool) = &state.db {
        if let Ok(true) = Auth::attempt(pool, &session, &req.email, &req.password).await {
            logged_in = true;
        }
    } else {
        // Fallback for demo without DB
        if req.email == "admin@example.com" && req.password == "password" {
            // Manually set session for demo
            session.insert("user_id", 1).await.unwrap();
            logged_in = true;
        }
    }

    if logged_in {
        Flash::success(&session, "Welcome back!").await;
        return Redirect::to("/");
    }

    Flash::error(&session, "Invalid credentials").await;
    Redirect::to("/login")
}

pub async fn logout(session: Session) -> impl IntoResponse {
    Auth::logout(&session).await;
    Flash::success(&session, "You have been logged out.").await;
    Redirect::to("/login")
}

pub async fn reset_password(
    FormRequest(req): FormRequest<ResetPasswordWithTokenRequest>
) -> impl IntoResponse {
    // If we get here, validation has passed!
    // 'req' is the ResetPasswordWithTokenRequest struct.

    dd!(req); // Dump the validated request data to prove it works

    Html("Password reset logic would go here".to_string())
}
