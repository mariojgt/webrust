use crate::prelude::*;

pub async fn index(inertia: Inertia) -> impl IntoResponse {
    inertia.render("Home", json!({
        "framework": "WebRust",
        "phpVersion": "None (It's Rust!)"
    }))
}
