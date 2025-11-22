# Application Lifecycle

Understanding how WebRust bootstraps and handles requests is key to building robust applications. This guide traces the journey from the moment you run the server to how a request is processed.

## 1. Bootstrapping (Startup)

The entry point of the application is `src/main.rs`. When you run `cargo run -- rune serve`, the following sequence occurs:

### A. Environment & CLI
1.  **Dotenv**: The `.env` file is loaded into environment variables.
2.  **CLI Parsing**: `clap` parses the command line arguments to determine which command to run (`serve`, `migrate`, etc.).

### B. Service Initialization
If the `serve` command is selected, the application initializes core services:

1.  **Logging**: The `tracing` subscriber is set up based on `LOG_CHANNEL` (stack, single, stdout).
2.  **Database**: A connection pool (`sqlx::Pool`) is created. If it fails, the app logs a warning but continues (allowing the app to run without a DB).
3.  **Cache**: The cache driver is initialized (Redis, File, or Memory) based on `CACHE_DRIVER`.
4.  **Templates**: Tera templates are compiled from the `templates/` directory.

### C. State Construction
All these services are bundled into the `AppState` struct:

```rust
pub struct AppState {
    pub db: Option<DbPool>,
    pub templates: Arc<Tera>,
    pub config: Arc<Config>,
    pub cache: Cache,
}
```

### D. Server Start
Finally, the `router` is built, and the Axum server binds to the configured host and port (default: `127.0.0.1:8000`).

---

## 2. The Router

The router is defined in `src/routes/mod.rs`. It acts as the traffic controller for your application.

### Route Groups
WebRust separates routes into two main groups:

1.  **Web Routes** (`src/routes/web.rs`):
    -   Intended for browser-based interaction.
    -   **Middleware**: CSRF Protection, Inertia.js Shared Data.
    -   **Session**: Cookies are encrypted and managed.

2.  **API Routes** (`src/routes/api.rs`):
    -   Intended for external consumers or mobile apps.
    -   **Middleware**: CORS (Cross-Origin Resource Sharing).
    -   **Stateless**: Typically uses token-based auth (though sessions are available globally).

### Static Files
The `public/` directory is served automatically at the root, allowing you to host assets like images, CSS, and JS.

---

## 3. Request Lifecycle

When a request hits your application, it flows through layers of middleware before reaching your controller.

### Step 1: Global Middleware
Every request passes through these layers first:
1.  **Compression**: Gzip/Brotli compression for smaller responses.
2.  **Logging**: The request method, path, and duration are logged.
3.  **Panic Handler**: Catches application panics and returns a 500 error instead of crashing the server.
4.  **Session**: The session is loaded from the store (Database or Memory) and attached to the request.

### Step 2: Route Matching & Group Middleware
Axum matches the URL to a defined route.
-   **If Web**:
    -   **CSRF**: Validates the `X-CSRF-TOKEN` or `csrf_token` form field for POST/PUT/DELETE requests.
    -   **Inertia**: Injects shared data (like user info or flash messages) into the Inertia context.
-   **If API**:
    -   **CORS**: Checks if the request origin is allowed.

### Step 3: The Controller
Your handler function executes. It can extract data using Axum's powerful extractors:

```rust
pub async fn update(
    State(state): State<AppState>,  // Access AppState
    session: Session,               // Access Session
    Path(id): Path<i32>,            // URL Parameters
    Form(input): Form<UpdateUser>,  // Form Data
) -> impl IntoResponse {
    // Your logic here
}
```

### Step 4: The Response
The controller returns a response (HTML, JSON, or Inertia). The response flows back up through the middleware stack (where compression happens) and is sent to the client.

---

## 4. Shutdown

When you press `Ctrl+C` or send a `SIGTERM`:
1.  The web server stops accepting new connections.
2.  Active requests are allowed to finish (graceful shutdown).
3.  Database connections are closed.
4.  The application process exits.
