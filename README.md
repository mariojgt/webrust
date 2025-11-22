# WebRust – Laravel-style Rust mini framework (with **Rune** CLI)

You wanted: *"a Rust framework like Laravel, with middleware, validation,
models, routes, views, controllers and artisan-style commands, but called **rune**."*

Here is your **WebRust starter** – a small but real codebase using:

- [Axum] for routing & middleware
- [Tera] for Blade-like templates (`.rune.html`)
- [SQLx] for MySQL
- [validator] for request validation
- [Clap] for an **artisan-like CLI** called **Rune**

---

## 🔧 0. Quick one-line setup (Rune script)

From the project root:

```bash
cargo run -- rune setup
```

This will:

- Check that the **DATABASE_URL** in `.env` works
- Create a `storage/` directory
- Print confirmation that WebRust is ready

(Details below if you want to do it step by step.)

---

## 1. Install prerequisites

- Rust (`rustup` + `cargo`)
- MySQL

Create a database:

```sql
CREATE DATABASE webrust_app CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

---

## 2. Configure environment

Copy the example env:

```bash
cp .env.example .env
```

Edit `.env` and set your MySQL credentials:

```env
DATABASE_URL=mysql://username:password@localhost:3306/webrust_app
```

---

## 3. Run the Rune setup script

```bash
cargo run -- rune setup
```

What it does:

- Connects to the database using `DATABASE_URL`
- Runs a cheap `SELECT 1` to confirm everything works
- Creates `storage/` folder (for logs, cache, etc. later)
- Prints:

```text
🔧 Running WebRust setup...
✅ Database connection OK
✅ storage/ directory ready
✨ Setup complete. You can now run `cargo run -- rune serve`
```

---

## 4. Run the dev server (like `php artisan serve`)

```bash
cargo run -- rune dev
```

This starts the server with **hot reloading**. It watches your `src/` and `templates/` directories and automatically restarts when you save changes.

By default it binds to `127.0.0.1:8000`.
You’ll see something like:

```text
🚀 Starting WebRust Dev Server...
📍 Listening on http://127.0.0.1:8000
💾 Watching for changes in src/ and templates/...
```

You can customize the address:

```bash
cargo run -- rune dev --host 0.0.0.0 --port 9000
```

### Stopping the server
To stop the server, simply press `Ctrl+C` in your terminal.

### Production Mode
For production (no hot reload, optimized build), use:
```bash
cargo run --release -- rune serve
```

---

## 5. Rune CLI – your artisan replacement

All CLI commands are under `rune`:

```bash
cargo run -- rune <command> [options]
```

### Available commands

- `rune setup` – run the initial setup (DB check + storage)
- `rune dev` – start the development server with hot reload
- `rune serve` – start the production HTTP server
- `rune make-controller <Name>` – generate a controller scaffold
- `rune make-model <Name>` – generate a model scaffold
- `rune migrate` – run database migrations

Examples:

```bash
# setup
cargo run -- rune setup

# dev server
cargo run -- rune dev

# generate Blog controller
cargo run -- rune make-controller Blog

# generate Post model
cargo run -- rune make-model Post

# run migrations
cargo run -- rune migrate
```

`rune make-controller Blog` will:

- Create `src/controllers/blog.rs`
- Ensure `src/controllers/mod.rs` has `pub mod blog;`
- Expect a template at `templates/blog/index.rune.html`

`rune make-model Post` will:
- Create `src/models/post.rs`
- Ensure `src/models/mod.rs` has `pub mod post;`

---

## 6. Project structure (Laravel mapping)

```text
src/
  main.rs          # parses Rune CLI, bootstraps server
  cli.rs           # rune commands (serve, setup, make-controller)
  framework.rs     # AppState, DB & Tera builders
  routes.rs        # defines all routes
  http/
    middleware.rs  # example log_request middleware
  controllers/
    mod.rs
    home.rs        # Home::index (GET /)
    users.rs       # Users::index (GET /users)
    contact.rs     # Contact::submit (POST /contact) with validation
  models/
    mod.rs
    user.rs        # User model using SQLx + chrono
templates/
  layout.rune.html       # base layout (like Blade layout)
  home/index.rune.html   # home view
  users/index.rune.html  # users list view
public/
  css/             # static CSS files
  js/              # static JS files
  images/          # static images
.env.example
README.md
```

**Laravel → WebRust**

- `routes/web.php` → `src/routes.rs`
- Controllers → `src/controllers/*`
- Middleware → `src/http/middleware.rs`
- FormRequests / validation → `controllers/contact.rs` (`ContactForm + validator`)
- Eloquent models → `src/models/*` (SQLx models)
- Blade views → `templates/**/*.rune.html`
- `public/` → `public/` (Static assets)
- `php artisan` → `cargo run -- rune <command>`

---

## 7. Middleware example

`src/http/middleware.rs`:

```rust
pub async fn log_request(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    info!(?method, ?uri, ?elapsed, "handled request");

    response
}
```

Applied in `src/routes.rs`:

```rust
Router::new()
    .route("/", get(home_index))
    .layer(axum::middleware::from_fn(log_request))
```

---

## 8. Validation example (`POST /contact`)

`controllers/contact.rs` shows how to validate JSON using `validator`:

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct ContactForm {
    #[validate(length(min = 3))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 10))]
    pub message: String,
}
```

- On success → `200 OK` with `{ ok: true, message: ... }`
- On failure → `422 Unprocessable Entity` with a map of field errors

---

## 9. Model + view example (`GET /users`)

Create a `users` table:

```sql
CREATE TABLE users (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

`src/models/user.rs`:

```rust
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
```

`GET /users` uses `User::all(&pool)` and renders `templates/users/index.rune.html`.

## 10. Debugging (dd, dump, debug)

WebRust includes Laravel-style debugging helpers:

```rust
use crate::prelude::*;

// Dump and Die (stops execution)
dd!(user);

// Dump and Continue
dump!(user);

// Labeled Debug
debug!("User Info", user);
```

See [DEBUG_QUICK_REF.md](DEBUG_QUICK_REF.md) for more details.

---

From here you can:

- Add more Rune commands: `make:model`, `make:view`, `make:migration`
- Extract DB logic into repositories
- Implement auth guards & middleware
- Add a `view!()` macro to simplify controllers

This is your **starting point** to grow WebRust into a full framework while learning Rust.
