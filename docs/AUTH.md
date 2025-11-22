# 🔐 Authentication

WebRust provides a quick way to scaffold a complete authentication system using the `rune make:auth` command. This includes login, registration, and logout functionality, along with the necessary views and routes.

## Scaffolding Auth

To generate the authentication controllers, views, and requests, run:

```bash
cargo run -- rune make:auth
```

This command will create:
- `src/controllers/auth.rs`: Handles login, register, and logout logic.
- `src/requests/auth.rs`: Validation structs for login and registration forms.
- `src/routes/auth.rs`: Defines the auth routes (`/login`, `/register`, `/logout`).
- `templates/auth/`: Login and Register HTML templates.
- `templates/dashboard.rune.html`: A protected dashboard view.

## Registering Routes

After running the command, you must manually register the new routes in `src/routes/mod.rs`.

1.  Open `src/routes/mod.rs`.
2.  Add `pub mod auth;` at the top.
3.  Merge the auth routes into your main router:

```rust
// src/routes/mod.rs

pub mod auth; // <--- Add this

pub fn router(state: AppState) -> Router {
    // ...
    Router::new()
        .merge(web_routes)
        .merge(api_routes)
        .merge(auth::routes(state.clone())) // <--- Add this
        // ...
}
```

## Protecting Routes

To protect routes so only logged-in users can access them, you can use the `Auth` service in your controller or middleware.

### Manual Check in Controller

```rust
use crate::services::auth::Auth;

pub async fn dashboard(session: Session) -> impl IntoResponse {
    if !Auth::check(&session).await {
        return Redirect::to("/login").into_response();
    }

    // Render dashboard...
}
```

### Middleware (Recommended)

You can create an `auth_middleware` to protect a group of routes.

```rust
// src/http/middleware/auth.rs
use axum::{http::Request, middleware::Next, response::{Redirect, Response, IntoResponse}};
use tower_sessions::Session;
use crate::services::auth::Auth;

pub async fn require_auth<B>(session: Session, req: Request<B>, next: Next<B>) -> Response {
    if Auth::check(&session).await {
        next.run(req).await
    } else {
        Redirect::to("/login").into_response()
    }
}
```

Then apply it to your routes:

```rust
.route("/dashboard", get(dashboard))
.layer(axum::middleware::from_fn(require_auth))
```

## Hashing

WebRust uses `bcrypt` for password hashing via the `src/services/hash.rs` service.

```rust
use crate::services::hash;

let hashed = hash::make("secret-password").unwrap();
let is_valid = hash::check("secret-password", &hashed);
```
