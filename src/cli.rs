use std::fs;
use std::io::{self, Write};
use std::path::Path;

use clap::{Parser, Subcommand};

use crate::framework;

#[derive(Parser, Debug)]
#[command(name = "webrust")]
#[command(about = "WebRust – Laravel-style framework in Rust (Rune CLI)")]
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
    MakeController {
        /// Name of the controller (e.g. Home, UserProfile)
        name: String,
    },

    /// Generate a new model file
    MakeModel {
        /// Name of the model (e.g. User, Post)
        name: String,
    },

    /// Generate a new middleware
    MakeMiddleware {
        /// Name of the middleware (e.g. Auth, Cors)
        name: String,
    },

    /// Generate a new form request
    MakeRequest {
        /// Name of the request (e.g. LoginRequest)
        name: String,
    },

    /// Create a new migration file
    MakeMigration {
        /// Name of the migration (e.g. create_users_table)
        name: String,
    },

    /// Run database migrations
    Migrate,

    /// Rollback the last database migration
    MigrateRollback,
}

pub async fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Running WebRust setup...");

    // Check DB connection (optional)
    match framework::build_pool().await {
        Ok(pool) => {
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => println!("✅ Database connection OK"),
                Err(e) => {
                    println!("⚠️  Database connection failed: {}", e);
                    println!("   You can still run the server, but database features won't work.");
                }
            }
        }
        Err(e) => {
            println!("⚠️  Could not build database pool: {}", e);
            println!("   You can still run the server, but database features won't work.");
        }
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
use sqlx::mysql::MySqlPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct {struct_name} {{
    pub id: i64,
    // Add your fields here
    // pub name: String,
    // pub created_at: chrono::DateTime<chrono::Utc>,
}}

impl {struct_name} {{
    pub async fn all(pool: &MySqlPool) -> Result<Vec<Self>, sqlx::Error> {{
        sqlx::query_as::<_, Self>("SELECT * FROM {module_name}s")
            .fetch_all(pool)
            .await
    }}

    pub async fn find(pool: &MySqlPool, id: i64) -> Result<Option<Self>, sqlx::Error> {{
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

pub async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running migrations...");
    let status = std::process::Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Migrations completed successfully"),
        Ok(_) => println!("❌ Migrations failed"),
        Err(_) => println!("❌ Could not run 'sqlx'. Is it installed? (cargo install sqlx-cli)"),
    }
    Ok(())
}

pub fn make_migration(name: &str) -> io::Result<()> {
    let now = chrono::Utc::now();
    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    let filename = format!("{}_{}.sql", timestamp, to_snake_case(name));

    let migrations_dir = Path::new("migrations");
    fs::create_dir_all(&migrations_dir)?;

    let file_path = migrations_dir.join(&filename);

    let contents = "-- Add migration script here\n";

    fs::write(&file_path, contents)?;
    println!("Created migration: {:?}", file_path);

    Ok(())
}

pub async fn rollback_migrations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rolling back migrations...");
    let status = std::process::Command::new("sqlx")
        .arg("migrate")
        .arg("revert")
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ Rollback completed successfully"),
        Ok(_) => println!("❌ Rollback failed"),
        Err(_) => println!("❌ Could not run 'sqlx'. Is it installed? (cargo install sqlx-cli)"),
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
