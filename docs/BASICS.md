# 🧱 Controllers, Views & APIs

This guide explains how to create controllers, render views, and build JSON APIs in WebRust.

## 1. Creating a Controller

You can generate a controller using the Rune CLI:

```bash
cargo run -- rune make:controller Post
```

This creates `src/controllers/post.rs` and automatically registers it in `src/controllers/mod.rs`.

### Manual Creation

If you prefer to create it manually:

1.  Create `src/controllers/post.rs`.
2.  Add `pub mod post;` to `src/controllers/mod.rs`.

## 2. Returning Views (HTML)

To return an HTML view, your controller function should return `Html<String>`. You can use the `view()` helper (if available) or the `state.templates.render()` method.

### Example: `src/controllers/post.rs`

```rust
use axum::{extract::State, response::Html};
use tera::Context;
use crate::framework::AppState;

// GET /posts
pub async fn index(State(state): State<AppState>) -> Html<String> {
    // 1. Create context for the template
    let mut ctx = Context::new();
    ctx.insert("title", "All Posts");
    ctx.insert("posts", &vec!["Post 1", "Post 2"]);

    // 2. Render the template
    let body = state.templates.render("posts/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {}", err));

    Html(body)
}
```

### The View File

Create `templates/posts/index.rune.html`:

```html
{% extends "layout.rune.html" %}

{% block content %}
    <h1>{{ title }}</h1>
    <ul>
    {% for post in posts %}
        <li>{{ post }}</li>
    {% endfor %}
    </ul>
{% endblock %}
```

### Registering the Route

Open `src/routes/web.rs` and register the route:

```rust
use crate::controllers::post; // Import the controller

pub fn web(state: AppState) -> Router {
    route::web()
        .get("/posts", post::index) // Register the route
        // ... other routes
        .build()
        .with_state(state)
}
```

## 3. Creating an API Controller (JSON)

To return JSON, use `axum::Json` as the return type.

### Example: `src/controllers/api/post_api.rs`

(You might want to organize API controllers in a subfolder, e.g., `src/controllers/api/`).

```rust
use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use crate::framework::AppState;

// GET /api/posts
pub async fn index() -> Json<Value> {
    Json(json!({
        "status": "success",
        "data": [
            { "id": 1, "title": "Hello World" },
            { "id": 2, "title": "WebRust is cool" }
        ]
    }))
}
```

### Registering the API Route

Open `src/routes/api.rs`:

```rust
use crate::controllers::api::post_api;

pub fn api(state: AppState) -> Router {
    route::api()
        .get("/posts", post_api::index) // Becomes GET /api/posts
        .build()
        .with_state(state)
}
```

## 4. Route Parameters

To capture URL segments (like `/posts/1`), use `axum::extract::Path`.

```rust
use axum::extract::Path;

// GET /posts/:id
pub async fn show(Path(id): Path<i64>, State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("post_id", &id);

    // ... fetch from DB using id ...

    let body = state.templates.render("posts/show.rune.html", &ctx).unwrap();
    Html(body)
}
```

Register it in `routes/web.rs`:

```rust
.get("/posts/:id", post::show)
```

## 5. Accessing the Database

Access the database pool via `state.db`.

```rust
use crate::models::post::Post;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let pool = state.db.as_ref().expect("Database not connected");

    let posts = Post::all(pool).await.unwrap();

    let mut ctx = Context::new();
    ctx.insert("posts", &posts);

    // ... render
}
```
