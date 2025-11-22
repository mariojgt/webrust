# WebRust Routes - Laravel-Style Routing

WebRust now supports Laravel-style route definition with a clean, fluent API.

## Quick Start

### Route files

Routes are organized in separate files, similar to Laravel:

- `src/routes/web.rs` - Web routes (HTML responses)
- `src/routes/api.rs` - API routes (JSON responses)

### Defining Routes

In `src/routes/web.rs`:

```rust
use crate::route;
use crate::framework::AppState;
use crate::controllers::{home, users};

pub fn web(state: AppState) -> Router {
    route::web()
        .get("/", home::index)
        .get("/users", users::index)
        .post("/contact", contact::submit)
        .build()
        .with_state(state)
}
```

## Route Methods

The fluent route builder supports:

- `.get(path, handler)`
- `.post(path, handler)`
- `.put(path, handler)`
- `.patch(path, handler)`
- `.delete(path, handler)`

Example:

```rust
route::web()
    .get("/", home::index)
    .post("/users", users::store)
    .put("/users/:id", users::update)
    .delete("/users/:id", users::destroy)
    .build()
    .with_state(state)
```

## Route Groups with Prefixes

Create grouped routes with a common prefix:

```rust
route::group("/admin")
    .get("/dashboard", admin::dashboard)
    .get("/users", admin::users::index)
    .post("/users", admin::users::store)
    .build()
    .with_state(state)
```

## API Routes

For API routes with `/api` prefix:

```rust
route::api()
    .get("/users", api::users::index)
    .post("/users", api::users::store)
    .get("/users/:id", api::users::show)
    .build()
    .with_state(state)
```

## Controller Example

Create a controller (or use `cargo run -- rune make-controller Name`):

```rust
// src/controllers/users.rs
use axum::{extract::State, response::Html};
use tera::Context;
use crate::framework::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("title", "Users");

    let body = state
        .templates
        .render("users/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Error: {}", err));

    Html(body)
}
```

## Middleware

Global middleware is applied in `src/routes/mod.rs`:

```rust
pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(web::web(state.clone()))
        .merge(api::api(state))
        .layer(axum::middleware::from_fn(log_request))
}
```

## Benefits Over Direct Routing

✅ Clean, Laravel-like syntax
✅ Easy to organize routes by domain
✅ Type-safe route definitions
✅ Fluent builder pattern
✅ Supports prefixes and route groups
✅ Minimal boilerplate
