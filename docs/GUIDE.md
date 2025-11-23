# WebRust Guide

WebRust is a Rust web framework designed to feel like Laravel. It provides a familiar developer experience with the performance and safety of Rust.

## 🚀 Getting Started

### Development Server
Run the development server with hot reloading (auto-restart on file changes):
```bash
cargo run -- rune dev
```
Or specify a port:
```bash
cargo run -- rune dev --port 3000
```

### Production Server
Run the optimized production server:
```bash
cargo run --release -- rune serve
```

## 📂 Project Structure

- `src/controllers/`: Request handlers (like Laravel controllers)
- `src/models/`: Database models (using SQLx)
- `src/routes/`: Route definitions (`web.rs` and `api.rs`)
- `templates/`: HTML templates (using Tera, similar to Blade)
- `public/`: Static assets (CSS, JS, images) served at `/public`
- `storage/`: Storage for logs, uploads, etc.

## 🛠 CLI Commands (Rune)

WebRust comes with `rune`, an Artisan-like CLI tool.

### Create a Controller
```bash
cargo run -- rune make-controller UserProfile
```
Creates `src/controllers/user_profile.rs` with a basic `index` method.

### Create a Model
```bash
cargo run -- rune make-model Post
```
Creates `src/models/post.rs` with basic CRUD methods (`all`, `find`).

### Run Migrations
```bash
cargo run -- rune migrate
```
Runs pending database migrations. WebRust uses a custom runtime SQL migration system, so no recompilation is needed.

### Other Commands
- `rune make:migration <Name>` – create a new SQL migration file
- `rune migrate:rollback` – rollback the last batch of migrations
- `rune make:auth` – scaffold authentication (login/register)
- `rune make:package <Name>` – scaffold a new package
- `rune queue:work` – start the queue worker
- `rune schedule:run` – run the scheduler

## 📡 Routing

Routes are defined in `src/routes/web.rs` and `src/routes/api.rs` using a fluent API.

```rust
use crate::route;

pub fn web(state: AppState) -> Router<AppState> {
    route::web()
        .get("/", home::index)
        .get("/about", home::about)
        .post("/contact", contact::submit)
        .build()
}
```

## 🗄 Database

WebRust uses **SQLx** for database interaction. It's type-safe and async.

### Example Model Usage
```rust
// In a controller
pub async fn show(State(state): State<AppState>, Path(id): Path<i64>) -> Html<String> {
    let user = User::find(&state.db.as_ref().unwrap(), id).await.unwrap();
    // ... render template
}
```

## 🎨 Templates

Templates live in `templates/` and use **Tera** syntax (Jinja2/Blade-like).

```html
<!-- templates/home/index.rune.html -->
{% extends "layout.rune.html" %}

{% block content %}
    <h1>Hello, {{ name }}!</h1>
{% endblock %}
```

## 📦 Static Files

Place your CSS, JS, and images in the `public/` directory.
They are accessible via `/public/...`.

Example: `public/css/app.css` -> `http://localhost:8000/public/css/app.css`
