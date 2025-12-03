# Controllers

Controllers are the heart of your application's logic. They handle incoming HTTP requests and return responses, often by rendering templates or returning JSON.

## Creating Controllers

You can generate a new controller using the Rune CLI:

```bash
cargo run -- rune make:controller User
```

This will create a file at `src/controllers/user.rs` with a basic structure.

### Resource Controllers

If you need a controller that handles full CRUD operations (Create, Read, Update, Delete), you can generate a resource controller:

```bash
cargo run -- rune make:resource Post
```

This will create:
- A controller at `src/controllers/post.rs` with methods for `index`, `create`, `store`, `show`, `edit`, `update`, and `destroy`.
- A route file at `src/routes/post.rs`.
- A set of templates in `templates/post/`.

## Controller Structure

A typical controller function looks like this:

```rust
use axum::{extract::State, response::Html};
use tera::Context;
use crate::framework::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("title", "Home Page");

    let body = state
        .templates
        .render("home/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {}", err));

    Html(body)
}
```

### Dependency Injection

WebRust uses Axum's extractors to inject dependencies into your controller methods.

- `State(state)`: Access the application state (database, cache, templates).
- `Path(id)`: Access URL parameters.
- `Query(params)`: Access query string parameters.
- `Form(payload)`: Access form data.
- `Json(payload)`: Access JSON request bodies.

## Registering Routes

After creating a controller, you need to register its routes.

1.  Open `src/routes/mod.rs` (or create a new route file).
2.  Import your controller.
3.  Define the route.

```rust
use crate::controllers::user;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users", get(user::index))
        .route("/users/:id", get(user::show))
}
```
