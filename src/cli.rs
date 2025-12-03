use std::fs;
use std::io::{self, Write};
use std::path::Path;

use clap::{Parser, Subcommand};

use crate::framework;

#[derive(Parser, Debug)]
#[command(name = "webrust")]
#[command(about = "WebRust – Laravel-inspired framework in Rust (Rune CLI)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Rune – artisan-like CLI
    Rune {
        #[command(subcommand)]
        rune: RuneCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum RuneCommand {
    /// Run the HTTP server (like `php artisan serve`)
    Serve {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(long, short, default_value = "8000")]
        port: u16,
    },

    /// Run the development server with auto-reload
    Dev {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(long, short, default_value = "8000")]
        port: u16,
    },

    /// Run initial setup (DB check, storage folder)
    Setup,

    /// Generate a new controller file (very small scaffold)
    #[command(name = "make:controller")]
    MakeController {
        /// Name of the controller (e.g. Home, UserProfile)
        name: String,
    },

    /// Generate a resource controller with full CRUD routes
    #[command(name = "make:resource")]
    MakeResource {
        /// Name of the resource (e.g. User, Post, Product)
        name: String,
        /// Include API routes as well
        #[arg(long)]
        api: bool,
    },

    /// Generate a new model file
    #[command(name = "make:model")]
    MakeModel {
        /// Name of the model (e.g. User, Post)
        name: String,
    },

    /// Generate a new middleware
    #[command(name = "make:middleware")]
    MakeMiddleware {
        /// Name of the middleware (e.g. Auth, Cors)
        name: String,
    },

    /// Generate a new form request
    #[command(name = "make:request")]
    MakeRequest {
        /// Name of the request (e.g. LoginRequest)
        name: String,
    },

    /// Create a new migration file
    #[command(name = "make:migration")]
    MakeMigration {
        /// Name of the migration (e.g. create_users_table)
        name: String,
    },

    /// Run database migrations
    Migrate,

    /// Seed the database with records
    #[command(name = "db:seed")]
    DbSeed {
        /// The class name of the root seeder
        #[arg(long, default_value = "DatabaseSeeder")]
        class: String,
    },

    /// Create a new seeder file
    #[command(name = "make:seeder")]
    MakeSeeder {
        /// Name of the seeder (e.g. UserSeeder)
        name: String,
    },

    /// Create a new notification class
    #[command(name = "make:notification")]
    MakeNotification {
        /// Name of the notification (e.g. WelcomeEmail)
        name: String,
    },

    /// Rollback the last database migration
    #[command(name = "migrate:rollback")]
    MigrateRollback,

    /// Start the queue worker
    #[command(name = "queue:work")]
    QueueWork {
        /// The name of the queue to work
        #[arg(long, default_value = "default")]
        queue: String,
    },

    /// Run the scheduled tasks
    #[command(name = "schedule:run")]
    ScheduleRun,

    /// Scaffold basic login and registration views and routes
    #[command(name = "make:auth")]
    MakeAuth,

    /// Create a new package scaffold
    #[command(name = "make:package")]
    MakePackage {
        /// Name of the package (e.g. blog, admin-panel)
        name: String,
    },

    /// Create a new custom command
    #[command(name = "make:command")]
    MakeCommand {
        /// Name of the command (e.g. SendEmails)
        name: String,
    },

    /// Open interactive Tinker REPL shell
    Tinker,

    /// List all application routes
    #[command(name = "route:list")]
    RouteList,

    /// List available migrations
    #[command(name = "migration:list")]
    MigrationList,

    /// Run a custom command
    #[command(external_subcommand)]
    External(Vec<String>),
}

pub async fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Running WebRust setup...");

    // Check DB connection (optional)
    let db_manager = framework::build_database_manager().await;

    if let Some(pool) = db_manager.default_connection() {
        match sqlx::query("SELECT 1").execute(pool).await {
            Ok(_) => println!("✅ Database connection OK"),
            Err(e) => {
                println!("⚠️  Database connection failed: {}", e);
                println!("   You can still run the server, but database features won't work.");
            }
        }
    } else {
        println!("⚠️  Could not connect to default database.");
        println!("   You can still run the server, but database features won't work.");
    }

    // Prepare storage directory
    fs::create_dir_all("storage")?;
    println!("✅ storage/ directory ready");

    println!("✨ Setup complete. You can now run `cargo run -- rune serve`");
    Ok(())
}

pub fn make_controller(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = format!("{name}Controller");

    let controllers_dir = Path::new("src").join("controllers");
    fs::create_dir_all(&controllers_dir)?;

    let file_path = controllers_dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Controller file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use axum::{{extract::State, response::Html}};
use tera::Context;

use crate::framework::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {{
    let mut ctx = Context::new();
    ctx.insert("title", "{struct_name}");
    ctx.insert("message", "You just generated this controller using the Rune CLI.");

    let body = state
        .templates
        .render("{module_name}/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {{err}}"));

    Html(body)
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created controller: {:?}", file_path);
    }

    // make sure mod.rs exposes this controller module
    let mod_path = controllers_dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};
");

    let mut current = String::new();
    if mod_path.exists() {
        current = fs::read_to_string(&mod_path)?;
    }

    if !current.contains(&mod_line) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mod_path)?;
        file.write_all(mod_line.as_bytes())?;
        println!("Updated controllers/mod.rs");
    }

    Ok(())
}

pub fn make_model(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);

    let models_dir = Path::new("src").join("models");
    fs::create_dir_all(&models_dir)?;

    let file_path = models_dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Model file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use serde::{{Deserialize, Serialize}};
use sqlx::FromRow;
use crate::database::DbPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct {struct_name} {{
    pub id: i64,
    // Add your fields here
    // pub name: String,
    // pub created_at: chrono::DateTime<chrono::Utc>,
}}

impl {struct_name} {{
    pub async fn all(pool: &DbPool) -> Result<Vec<Self>, sqlx::Error> {{
        sqlx::query_as::<_, Self>("SELECT * FROM {module_name}s")
            .fetch_all(pool)
            .await
    }}

    pub async fn find(pool: &DbPool, id: i64) -> Result<Option<Self>, sqlx::Error> {{
        sqlx::query_as::<_, Self>("SELECT * FROM {module_name}s WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }}
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created model: {:?}", file_path);
    }

    // make sure mod.rs exposes this model module
    let mod_path = models_dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");

    let mut current = String::new();
    if mod_path.exists() {
        current = fs::read_to_string(&mod_path)?;
    }

    if !current.contains(&mod_line) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mod_path)?;
        file.write_all(mod_line.as_bytes())?;
        println!("Updated models/mod.rs");
    }

    Ok(())
}

pub fn make_middleware(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);

    let dir = Path::new("src").join("http").join("middleware");
    fs::create_dir_all(&dir)?;

    let file_path = dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Middleware file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use axum::{{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
}};

pub async fn {module_name}(req: Request<Body>, next: Next) -> Response {{
    // Logic before request
    // println!("Middleware {struct_name} running");

    let response = next.run(req).await;

    // Logic after request

    response
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created middleware: {:?}", file_path);
    }

    // Update mod.rs
    let mod_path = dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");
    append_to_mod_file(&mod_path, &mod_line)?;

    Ok(())
}

pub fn make_request(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);

    let dir = Path::new("src").join("requests");
    fs::create_dir_all(&dir)?;

    let file_path = dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Request file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct {struct_name} {{
    // Add your validation rules here
    // #[validate(email(message = "Invalid email"))]
    // pub email: String,
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created request: {:?}", file_path);
    }

    // Update mod.rs
    let mod_path = dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");
    append_to_mod_file(&mod_path, &mod_line)?;

    Ok(())
}

pub fn make_auth() -> io::Result<()> {
    println!("🔐 Scaffolding authentication...");

    // 1. Create Requests
    let requests_dir = Path::new("src").join("requests");
    fs::create_dir_all(&requests_dir)?;
    let auth_request_path = requests_dir.join("auth.rs");

    let auth_request_content = r#"
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub password_confirmation: String,
}
"#;
    fs::write(&auth_request_path, auth_request_content.trim_start())?;
    println!("✅ Created src/requests/auth.rs");
    append_to_mod_file(&requests_dir.join("mod.rs"), "pub mod auth;\n")?;

    // 2. Create Controller
    let controllers_dir = Path::new("src").join("controllers");
    fs::create_dir_all(&controllers_dir)?;
    let auth_controller_path = controllers_dir.join("auth.rs");

    let auth_controller_content = r#"
use axum::{
    extract::State,
    response::{Html, Redirect, Response, IntoResponse},
    Form,
};
use tera::Context;
use validator::Validate;
use tower_sessions::Session;

use crate::framework::AppState;
use crate::requests::auth::{LoginRequest, RegisterRequest};
use crate::services::{auth::Auth, hash, flash::Flash};
use crate::models::user::User;
use crate::orbit::Orbit;
use crate::prelude::*;

pub async fn login_form(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Login");

    // Pass flash messages to view
    if let Some(msg) = Flash::get(&session, "error").await {
        ctx.insert("error", &msg);
    }
    if let Some(msg) = Flash::get(&session, "success").await {
        ctx.insert("success", &msg);
    }

    let body = state.templates.render("auth/login.rune.html", &ctx).unwrap();
    Html(body)
}

pub async fn register_form(State(state): State<AppState>, session: Session) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Register");

    if let Some(msg) = Flash::get(&session, "error").await {
        ctx.insert("error", &msg);
    }

    let body = state.templates.render("auth/register.rune.html", &ctx).unwrap();
    Html(body)
}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    Form(payload): Form<LoginRequest>,
) -> Response {
    if let Err(e) = payload.validate() {
        Flash::error(&session, "Validation failed").await;
        return Redirect::to("/login").into_response();
    }

    if let Some(pool) = &state.db {
        match Auth::attempt(pool, &session, &payload.email, &payload.password).await {
            Ok(true) => {
                Flash::success(&session, "Welcome back!").await;
                Redirect::to("/dashboard").into_response()
            }
            _ => {
                Flash::error(&session, "Invalid credentials").await;
                Redirect::to("/login").into_response()
            }
        }
    } else {
        Flash::error(&session, "Database not connected").await;
        Redirect::to("/login").into_response()
    }
}

pub async fn register(
    State(state): State<AppState>,
    session: Session,
    Form(payload): Form<RegisterRequest>,
) -> Response {
    if let Err(e) = payload.validate() {
        Flash::error(&session, "Validation failed").await;
        return Redirect::to("/register").into_response();
    }

    if let Some(pool) = &state.db {
        // Check if user exists
        if let Ok(Some(_)) = User::find_by_email(pool, &payload.email).await {
            Flash::error(&session, "Email already taken").await;
            return Redirect::to("/register").into_response();
        }

        // Create user
        let hashed = hash::make(&payload.password).unwrap();

        // Using Orbit to create
        // Note: You might need to define a NewUser struct or use a map if your User struct has fields that are not in the form
        // For simplicity, we assume we can insert raw SQL or use a helper.
        // Here we'll use raw SQL for safety as User struct might have ID.

        let result = sqlx::query("INSERT INTO users (name, email, password, created_at) VALUES (?, ?, ?, NOW())")
            .bind(&payload.name)
            .bind(&payload.email)
            .bind(&hashed)
            .execute(pool)
            .await;

        match result {
            Ok(_) => {
                // Login immediately
                let _ = Auth::attempt(pool, &session, &payload.email, &payload.password).await;
                Flash::success(&session, "Account created!").await;
                Redirect::to("/dashboard").into_response()
            }
            Err(e) => {
                Flash::error(&session, &format!("Database error: {}", e)).await;
                Redirect::to("/register").into_response()
            }
        }
    } else {
        Flash::error(&session, "Database not connected").await;
        Redirect::to("/register").into_response()
    }
}

pub async fn logout(session: Session) -> impl IntoResponse {
    Auth::logout(&session).await;
    Flash::success(&session, "Logged out successfully").await;
    Redirect::to("/login")
}
"#;
    fs::write(&auth_controller_path, auth_controller_content.trim_start())?;
    println!("✅ Created src/controllers/auth.rs");
    append_to_mod_file(&controllers_dir.join("mod.rs"), "pub mod auth;\n")?;

    // 3. Create Routes
    let routes_dir = Path::new("src").join("routes");
    let auth_routes_path = routes_dir.join("auth.rs");
    let auth_routes_content = r#"
use axum::{
    routing::{get, post},
    Router,
};
use crate::framework::AppState;
use crate::controllers::auth;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", get(auth::login_form).post(auth::login))
        .route("/register", get(auth::register_form).post(auth::register))
        .route("/logout", post(auth::logout))
}
"#;
    fs::write(&auth_routes_path, auth_routes_content.trim_start())?;
    println!("✅ Created src/routes/auth.rs");
    // Note: User still needs to register this in src/routes/mod.rs manually or we try to inject it.

    // 4. Create Templates
    let templates_dir = Path::new("templates").join("auth");
    fs::create_dir_all(&templates_dir)?;

    let login_html = r#"
{% extends "layout.rune.html" %}

{% block content %}
<div style="max-width: 400px; margin: 2rem auto; padding: 2rem; border: 1px solid #ccc; border-radius: 8px;">
    <h2>Login</h2>

    {% if error %}
        <div style="color: red; margin-bottom: 1rem;">{{ error }}</div>
    {% endif %}
    {% if success %}
        <div style="color: green; margin-bottom: 1rem;">{{ success }}</div>
    {% endif %}

    <form action="/login" method="POST">
        <input type="hidden" name="csrf_token" value="{{ csrf_token }}">

        <div style="margin-bottom: 1rem;">
            <label>Email</label>
            <input type="email" name="email" required style="width: 100%; padding: 0.5rem;">
        </div>

        <div style="margin-bottom: 1rem;">
            <label>Password</label>
            <input type="password" name="password" required style="width: 100%; padding: 0.5rem;">
        </div>

        <button type="submit" style="width: 100%; padding: 0.5rem; background: #333; color: white; border: none; cursor: pointer;">Login</button>
    </form>

    <p style="margin-top: 1rem; text-align: center;">
        Don't have an account? <a href="/register">Register</a>
    </p>
</div>
{% endblock %}
"#;
    fs::write(templates_dir.join("login.rune.html"), login_html.trim_start())?;

    let register_html = r#"
{% extends "layout.rune.html" %}

{% block content %}
<div style="max-width: 400px; margin: 2rem auto; padding: 2rem; border: 1px solid #ccc; border-radius: 8px;">
    <h2>Register</h2>

    {% if error %}
        <div style="color: red; margin-bottom: 1rem;">{{ error }}</div>
    {% endif %}

    <form action="/register" method="POST">
        <input type="hidden" name="csrf_token" value="{{ csrf_token }}">

        <div style="margin-bottom: 1rem;">
            <label>Name</label>
            <input type="text" name="name" required style="width: 100%; padding: 0.5rem;">
        </div>

        <div style="margin-bottom: 1rem;">
            <label>Email</label>
            <input type="email" name="email" required style="width: 100%; padding: 0.5rem;">
        </div>

        <div style="margin-bottom: 1rem;">
            <label>Password</label>
            <input type="password" name="password" required style="width: 100%; padding: 0.5rem;">
        </div>

        <div style="margin-bottom: 1rem;">
            <label>Confirm Password</label>
            <input type="password" name="password_confirmation" required style="width: 100%; padding: 0.5rem;">
        </div>

        <button type="submit" style="width: 100%; padding: 0.5rem; background: #333; color: white; border: none; cursor: pointer;">Register</button>
    </form>

    <p style="margin-top: 1rem; text-align: center;">
        Already have an account? <a href="/login">Login</a>
    </p>
</div>
{% endblock %}
"#;
    fs::write(templates_dir.join("register.rune.html"), register_html.trim_start())?;
    println!("✅ Created templates/auth/");

    // Dashboard
    let dashboard_html = r#"
{% extends "layout.rune.html" %}

{% block content %}
<div style="max-width: 800px; margin: 2rem auto;">
    <h1>Dashboard</h1>
    <p>You are logged in!</p>

    <form action="/logout" method="POST">
        <input type="hidden" name="csrf_token" value="{{ csrf_token }}">
        <button type="submit">Logout</button>
    </form>
</div>
{% endblock %}
"#;
    fs::write(Path::new("templates").join("dashboard.rune.html"), dashboard_html.trim_start())?;
    println!("✅ Created templates/dashboard.rune.html");

    println!("\n⚠️  Action Required: Update src/routes/mod.rs");
    println!("Add the following lines to register the auth routes:");
    println!("    pub mod auth;");
    println!("    // Inside router function:");
    println!("    .merge(auth::routes(state.clone()))");

    Ok(())
}

pub fn make_package(name: &str) -> io::Result<()> {
    let package_name = to_snake_case(name);
    let package_dir = Path::new("packages").join(&package_name);
    let src_dir = package_dir.join("src");

    if package_dir.exists() {
        println!("Package directory already exists: {:?}", package_dir);
        return Ok(());
    }

    fs::create_dir_all(&src_dir)?;

    // 1. Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
serde = {{ version = "1.0", features = ["derive"] }}
webrust = {{ path = "../../" }}
"#
    );
    fs::write(package_dir.join("Cargo.toml"), cargo_toml)?;

    // 2. Create src/lib.rs
    let lib_rs = format!(
        r#"use axum::Router;
use webrust::framework::AppState;
use webrust::framework::WebRustPackage;

pub struct Package;

impl WebRustPackage for Package {{
    fn name(&self) -> &str {{
        "{package_name}"
    }}

    fn routes(&self, _state: AppState) -> Router<AppState> {{
        Router::new()
        // .route("/{package_name}", axum::routing::get(|| async {{ "Hello from {package_name}!" }}))
    }}
}}
"#
    );
    fs::write(src_dir.join("lib.rs"), lib_rs)?;

    println!("✅ Created package: {:?}", package_dir);
    println!("\nTo enable this package:");
    println!("1. Add it to [dependencies] in Cargo.toml:");
    println!("   {} = {{ path = \"packages/{}\" }}", package_name, package_name);
    println!("2. Register it in src/main.rs (or wherever you load packages).");

    Ok(())
}

pub fn make_command(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);

    let commands_dir = Path::new("src").join("commands");
    fs::create_dir_all(&commands_dir)?;

    let file_path = commands_dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Command file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use async_trait::async_trait;
use crate::services::console::Command;

pub struct {struct_name};

#[async_trait]
impl Command for {struct_name} {{
    fn name(&self) -> &str {{
        "{module_name}" // e.g. email:send
    }}

    fn description(&self) -> &str {{
        "Command description"
    }}

    async fn handle(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Running command: {{}}", self.name());
        println!("Args: {{:?}}", args);

        // Your logic here

        Ok(())
    }}
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created command: {:?}", file_path);
    }

    // Update mod.rs
    let mod_path = commands_dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");
    append_to_mod_file(&mod_path, &mod_line)?;

    // Auto-register in kernel()
    let register_line = format!(
        "    commands.insert(\"{}\".to_string(), Box::new({}::{}));",
        module_name.replace("_", ":"),
        module_name,
        struct_name
    );

    let mut mod_content = fs::read_to_string(&mod_path)?;
    if !mod_content.contains(&register_line) {
        // Look for the end of the kernel function to insert before the return
        if let Some(pos) = mod_content.rfind("commands\n}") {
             mod_content.insert_str(pos, &format!("{}\n\n    ", register_line.trim()));
             fs::write(&mod_path, mod_content)?;
             println!("✅ Automatically registered command in src/commands/mod.rs");
        } else {
             println!("\n⚠️  Could not auto-register. Please add this line to kernel():");
             println!("{}", register_line);
        }
    }

    Ok(())
}

fn append_to_mod_file(path: &Path, line: &str) -> io::Result<()> {
    let mut current = String::new();
    if path.exists() {
        current = fs::read_to_string(path)?;
    }

    if !current.contains(line) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(line.as_bytes())?;
        println!("Updated {:?}", path);
    }
    Ok(())
}

pub fn make_seeder(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);

    let seeders_dir = Path::new("src").join("database").join("seeders");
    fs::create_dir_all(&seeders_dir)?;

    let file_path = seeders_dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Seeder file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use async_trait::async_trait;
use crate::database::DatabaseManager;
use crate::database::seeder::Seeder;
use crate::services::factory::Factory;
// use crate::models::user::User;
// use crate::services::factory::UserFactory;

pub struct {struct_name};

#[async_trait]
impl Seeder for {struct_name} {{
    async fn run(&self, db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {{
        // Example: Create 10 users
        // UserFactory::new().create_many(10).await?;

        Ok(())
    }}
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created seeder: {:?}", file_path);
    }

    // Update mod.rs
    let mod_path = seeders_dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");
    append_to_mod_file(&mod_path, &mod_line)?;

    Ok(())
}

pub fn make_notification(name: &str) -> io::Result<()> {
    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);

    let notifications_dir = Path::new("src").join("notifications");
    fs::create_dir_all(&notifications_dir)?;

    let file_path = notifications_dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("Notification file already exists: {:?}", file_path);
    } else {
        let contents = format!(
            r#"
use async_trait::async_trait;
use crate::services::notification::{{Notification, Notifiable, DatabaseMessage}};
use crate::services::mail::MailMessage;
use serde_json::json;

pub struct {struct_name};

#[async_trait]
impl Notification for {struct_name} {{
    fn via(&self, _notifiable: &dyn Notifiable) -> Vec<String> {{
        vec!["mail".to_string()]
    }}

    fn to_mail(&self, notifiable: &dyn Notifiable) -> Option<MailMessage> {{
        Some(MailMessage {{
            to: notifiable.route_notification_for("mail")?,
            subject: "Notification Subject".to_string(),
            body: "Notification Body".to_string(),
        }})
    }}

    fn to_database(&self, _notifiable: &dyn Notifiable) -> Option<DatabaseMessage> {{
        Some(DatabaseMessage {{
            message: "Notification Message".to_string(),
            data: json!({{
                "key": "value"
            }}),
        }})
    }}
}}
"#
        );

        fs::write(&file_path, contents.trim_start())?;
        println!("Created notification: {:?}", file_path);
    }

    // Update mod.rs
    let mod_path = notifications_dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");
    append_to_mod_file(&mod_path, &mod_line)?;

    Ok(())
}

pub async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running migrations...");

    let db_manager = framework::build_database_manager().await;
    let migrator = crate::database::migrator::Migrator::new(db_manager);

    match migrator.run(Path::new("migrations")).await {
        Ok(_) => println!("✅ Migrations completed successfully"),
        Err(e) => println!("❌ Migrations failed: {}", e),
    }

    Ok(())
}

pub fn make_migration(name: &str) -> io::Result<()> {
    let now = chrono::Utc::now();
    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    let filename = format!("{}_{}.sql", timestamp, to_snake_case(name));
    let table_name = to_snake_case(name);

    let migrations_dir = Path::new("migrations");
    fs::create_dir_all(&migrations_dir)?;

    let file_path = migrations_dir.join(&filename);

    let contents = format!(
        r#"-- Migration: {name}
-- --- UP ---
CREATE TABLE {table_name} (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- --- DOWN ---
DROP TABLE IF EXISTS {table_name};
"#
    );

    fs::write(&file_path, contents.trim_start())?;
    println!("Created migration: {:?}", file_path);

    Ok(())
}

pub async fn rollback_migrations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rolling back migrations...");

    let db_manager = framework::build_database_manager().await;
    let migrator = crate::database::migrator::Migrator::new(db_manager);

    match migrator.rollback(Path::new("migrations")).await {
        Ok(_) => println!("✅ Rollback completed successfully"),
        Err(e) => println!("❌ Rollback failed: {}", e),
    }

    Ok(())
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;

    for c in s.chars() {
        if c.is_uppercase() {
            if prev_lower {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
            prev_lower = false;
        } else if c == '-' || c == ' ' {
            out.push('_');
            prev_lower = false;
        } else {
            out.push(c);
            prev_lower = c.is_lowercase();
        }
    }

    out
}

#[allow(dead_code)]
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-' || c == ' ')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

/// Generate a full resource controller with CRUD operations
pub fn make_resource(name: &str, api: bool) -> io::Result<()> {
    println!("📦 Generating resource controller for '{}'...", name);

    let module_name = to_snake_case(name);
    let struct_name = to_pascal_case(name);
    let plural_name = format!("{}s", module_name); // Simple pluralization

    let controllers_dir = Path::new("src").join("controllers");
    fs::create_dir_all(&controllers_dir)?;

    let file_path = controllers_dir.join(format!("{module_name}.rs"));

    if file_path.exists() {
        println!("❌ Controller file already exists: {:?}", file_path);
        return Ok(());
    }

    let contents = format!(
        r#"
use axum::{{
    extract::{{State, Path}},
    response::{{Html, IntoResponse, Response}},
    Form, Json,
}};
use tera::Context;
use serde::{{Deserialize, Serialize}};

use crate::framework::AppState;
use crate::prelude::*;

/// Resource controller for {struct_name}
/// Implements standard CRUD operations

/// Index - List all {plural_name}
pub async fn index(State(state): State<AppState>) -> impl IntoResponse {{
    let mut ctx = Context::new();
    ctx.insert("title", "{struct_name} List");

    let body = state
        .templates
        .render("{module_name}/index.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {{err}}"));

    Html(body)
}}

/// Create - Show creation form
pub async fn create(State(state): State<AppState>) -> impl IntoResponse {{
    let mut ctx = Context::new();
    ctx.insert("title", "Create {struct_name}");

    let body = state
        .templates
        .render("{module_name}/create.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {{err}}"));

    Html(body)
}}

/// Store - Create new {struct_name}
pub async fn store(State(state): State<AppState>) -> impl IntoResponse {{
    success_message("Resource created successfully")
}}

/// Show - Display a single {struct_name}
pub async fn show(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {{
    let mut ctx = Context::new();
    ctx.insert("title", "View {struct_name}");
    ctx.insert("id", &id);

    let body = state
        .templates
        .render("{module_name}/show.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {{err}}"));

    Html(body)
}}

/// Edit - Show edit form
pub async fn edit(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {{
    let mut ctx = Context::new();
    ctx.insert("title", "Edit {struct_name}");
    ctx.insert("id", &id);

    let body = state
        .templates
        .render("{module_name}/edit.rune.html", &ctx)
        .unwrap_or_else(|err| format!("Template error: {{err}}"));

    Html(body)
}}

/// Update - Update a {struct_name}
pub async fn update(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {{
    success_message("Resource updated successfully")
}}

/// Destroy - Delete a {struct_name}
pub async fn destroy(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {{
    success_message("Resource deleted successfully")
}}
"#
    );

    fs::write(&file_path, contents.trim_start())?;
    println!("✅ Created controller: {:?}", file_path);

    // Update mod.rs
    let mod_path = controllers_dir.join("mod.rs");
    let mod_line = format!("pub mod {module_name};\n");
    append_to_mod_file(&mod_path, &mod_line)?;

    // Create routes
    let routes_dir = Path::new("src").join("routes");
    let resource_routes_path = routes_dir.join(format!("{module_name}.rs"));

    let routes_content = format!(
        r#"
use axum::{{
    routing::{{get, post, put, delete}},
    Router,
}};
use crate::framework::AppState;
use crate::controllers::{module_name};

pub fn routes(state: AppState) -> Router<AppState> {{
    Router::new()
        .route("/{plural_name}", get({module_name}::index).post({module_name}::store))
        .route("/{plural_name}/create", get({module_name}::create))
        .route("/{plural_name}/:id", get({module_name}::show).put({module_name}::update).delete({module_name}::destroy))
        .route("/{plural_name}/:id/edit", get({module_name}::edit))
        .with_state(state)
}}
"#
    );

    fs::write(&resource_routes_path, routes_content.trim_start())?;
    println!("✅ Created routes: {:?}", resource_routes_path);

    // Create template directory
    let templates_dir = Path::new("templates").join(&module_name);
    fs::create_dir_all(&templates_dir)?;

    // Create basic templates
    let templates = vec![
        ("index.rune.html", format!(r#"
{{% extends "layout.rune.html" %}}

{{% block content %}}
<div class="container">
    <h1>{} List</h1>
    <a href="/{}/create">Create New</a>

    <table>
        <thead>
            <tr>
                <th>ID</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            <!-- Populate from database -->
        </tbody>
    </table>
</div>
{{% endblock %}}
"#, struct_name, plural_name)),
        ("create.rune.html", format!(r#"
{{% extends "layout.rune.html" %}}

{{% block content %}}
<div class="container">
    <h1>Create {}</h1>

    <form action="/{}" method="POST">
        {{% csrf_field %}}

        <!-- Add form fields here -->

        <button type="submit">Create</button>
    </form>
</div>
{{% endblock %}}
"#, struct_name, plural_name)),
        ("show.rune.html", format!(r#"
{{% extends "layout.rune.html" %}}

{{% block content %}}
<div class="container">
    <h1>View {}</h1>

    <a href="/{}/{{ id }}/edit">Edit</a>

    <!-- Display resource details -->
</div>
{{% endblock %}}
"#, struct_name, plural_name)),
        ("edit.rune.html", format!(r#"
{{% extends "layout.rune.html" %}}

{{% block content %}}
<div class="container">
    <h1>Edit {}</h1>

    <form action="/{}/{{ id }}" method="POST">
        {{% csrf_field %}}
        <input type="hidden" name="_method" value="PUT">

        <!-- Add form fields here -->

        <button type="submit">Update</button>
    </form>
</div>
{{% endblock %}}
"#, struct_name, plural_name)),
    ];

    for (filename, content) in templates {
        let template_path = templates_dir.join(filename);
        fs::write(&template_path, content)?;
        println!("✅ Created template: {:?}", template_path);
    }

    println!("✨ Resource controller created successfully!");
    println!("📝 Next steps:");
    println!("  1. Add the routes to src/routes/mod.rs");
    println!("  2. Create a model for this resource");
    println!("  3. Update the templates with your fields");
    println!("  4. Implement the controller logic");

    Ok(())
}
