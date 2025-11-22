use crate::prelude::*;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    // Example: Uncomment to debug
    // dd!("Home controller accessed!");

    let mut ctx = Context::new();
    ctx.insert("title", "WebRust – tiny Laravel-style framework in Rust");
    ctx.insert("message", "You are on the home page rendered with .rune.html templates.");


    let body = state
        .templates
        .render("home/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {err}"));

    Html(body)
}
