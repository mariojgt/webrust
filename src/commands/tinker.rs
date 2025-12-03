use std::io::{self, Write};
use crate::framework::AppState;

/// Interactive Tinker/REPL shell for debugging and prototyping
pub async fn tinker(app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║             🔧 WebRust Tinker REPL v1.0                    ║");
    println!("║                                                            ║");
    println!("║ Welcome to the WebRust interactive shell!                  ║");
    println!("║ Type 'help' for available commands or 'exit' to quit.      ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    loop {
        print!(">> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" | "q" => {
                println!("Goodbye! 👋");
                break;
            }
            "help" => print_help(),
            "clear" => {
                print!("{esc}[2J{esc}[H", esc = 27 as char);
                println!("🧹 Screen cleared");
            }
            cmd if cmd.starts_with("db:") => {
                execute_db_command(cmd, &app_state).await?;
            }
            cmd if cmd.starts_with("sql:") => {
                execute_sql_command(cmd, &app_state).await?;
            }
            cmd if cmd.starts_with("config:") => {
                execute_config_command(cmd);
            }
            cmd if cmd.starts_with("route:") => {
                execute_route_command(cmd);
            }
            "info" => print_app_info(&app_state),
            _ => {
                println!("❌ Unknown command: '{}'. Type 'help' for available commands.", input);
            }
        }
        println!();
    }

    Ok(())
}

fn print_help() {
    println!("📖 Available Tinker Commands:");
    println!();
    println!("  Database Commands:");
    println!("    db:tables              - List all database tables");
    println!("    db:table <name>        - Show table columns and info");
    println!("    db:count <table>       - Count rows in a table");
    println!();
    println!("  SQL Commands:");
    println!("    sql:execute <query>    - Execute raw SQL query");
    println!("    sql:last               - Show last executed query");
    println!();
    println!("  Configuration:");
    println!("    config:app             - Show application configuration");
    println!("    config:db              - Show database configuration");
    println!("    config:env             - Show environment variables");
    println!();
    println!("  Application Info:");
    println!("    route:list             - List all registered routes");
    println!("    info                   - Show application info");
    println!();
    println!("  General:");
    println!("    help                   - Show this help menu");
    println!("    clear                  - Clear the screen");
    println!("    exit/quit/q            - Exit Tinker");
    println!();
}

async fn execute_db_command(
    cmd: &str,
    app_state: &AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    match parts.get(0) {
        Some(&"db:tables") => {
            if let Some(pool) = app_state.db_manager.default_connection() {
                println!("📋 Database Tables:");
                let result: Vec<(String,)> =
                    sqlx::query_as("SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()")
                        .fetch_all(pool)
                        .await?;

                for (table,) in result {
                    println!("  • {}", table);
                }
            } else {
                println!("❌ No database connection available");
            }
        }
        Some(&"db:table") => {
            if let Some(table_name) = parts.get(1) {
                if let Some(pool) = app_state.db_manager.default_connection() {
                    println!("📊 Table: {}", table_name);
                    let result: Vec<(String, String, String)> = sqlx::query_as(
                        "SELECT COLUMN_NAME, COLUMN_TYPE, IS_NULLABLE FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?",
                    )
                    .bind(table_name)
                    .fetch_all(pool)
                    .await?;

                    println!("  Columns:");
                    for (col_name, col_type, nullable) in result {
                        let null_str = if nullable == "YES" { "nullable" } else { "NOT NULL" };
                        println!("    • {} {} ({})", col_name, col_type, null_str);
                    }
                } else {
                    println!("❌ No database connection available");
                }
            } else {
                println!("❌ Usage: db:table <table_name>");
            }
        }
        Some(&"db:count") => {
            if let Some(table_name) = parts.get(1) {
                if let Some(pool) = app_state.db_manager.default_connection() {
                    let query = format!("SELECT COUNT(*) as count FROM `{}`", table_name);
                    let result: (i64,) = sqlx::query_as(&query).fetch_one(pool).await?;
                    println!("📈 Table '{}' has {} rows", table_name, result.0);
                } else {
                    println!("❌ No database connection available");
                }
            } else {
                println!("❌ Usage: db:count <table_name>");
            }
        }
        _ => println!("❌ Unknown database command"),
    }

    Ok(())
}

async fn execute_sql_command(
    cmd: &str,
    app_state: &AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    let sql = cmd.strip_prefix("sql:execute ").unwrap_or("");

    if sql.is_empty() {
        println!("❌ Usage: sql:execute <query>");
        return Ok(());
    }

    if let Some(pool) = app_state.db_manager.default_connection() {
        match sqlx::query(sql).execute(pool).await {
            Ok(result) => {
                println!("✅ Query executed successfully");
                println!("   Rows affected: {}", result.rows_affected());
            }
            Err(e) => {
                println!("❌ Query failed: {}", e);
            }
        }
    } else {
        println!("❌ No database connection available");
    }

    Ok(())
}

fn execute_config_command(cmd: &str) {
    match cmd {
        "config:app" => {
            println!("🔧 Application Configuration:");
            println!("  • Framework: WebRust");
            println!("  • Version: 0.1.0");
            println!("  • Rust Edition: 2021");
            if let Ok(env) = std::env::var("APP_ENV") {
                println!("  • Environment: {}", env);
            }
            if let Ok(debug) = std::env::var("APP_DEBUG") {
                println!("  • Debug Mode: {}", debug);
            }
        }
        "config:db" => {
            println!("🗄️  Database Configuration:");
            if let Ok(url) = std::env::var("DATABASE_URL") {
                let masked = mask_db_url(&url);
                println!("  • Connection URL: {}", masked);
            }
        }
        "config:env" => {
            println!("🌍 Environment Variables:");
            for (key, value) in std::env::vars() {
                if key.starts_with("APP_") || key.starts_with("DATABASE_") {
                    let display_value = if key.contains("PASSWORD") {
                        "***".to_string()
                    } else {
                        value
                    };
                    println!("  • {}: {}", key, display_value);
                }
            }
        }
        _ => println!("❌ Unknown config command"),
    }
}

fn execute_route_command(cmd: &str) {
    match cmd {
        "route:list" => {
            println!("📍 Application Routes:");
            println!("  (Routes would be listed here - currently static example)");
            println!("  GET     /                   → home::index");
            println!("  GET     /users              → users::index");
            println!("  GET     /users/{{id}}        → users::show");
            println!("  POST    /users              → users::store");
            println!("  GET     /users/{{id}}/edit   → users::edit");
            println!("  PUT     /users/{{id}}        → users::update");
            println!("  DELETE  /users/{{id}}        → users::destroy");
        }
        _ => println!("❌ Unknown route command"),
    }
}

fn print_app_info(app_state: &AppState) {
    println!("ℹ️  Application Information:");
    println!("  • Framework: WebRust");
    println!("  • HTTP Server: Axum");
    println!("  • Templating: Tera");
    println!("  • Database: SQLx");

    if app_state.db_manager.default_connection().is_some() {
        println!("  • Database Connection: ✅ Active");
    } else {
        println!("  • Database Connection: ❌ Not configured");
    }

    println!("  • Caching: {} configured",
        if true { "Redis" } else { "No cache" }
    );
}

fn mask_db_url(url: &str) -> String {
    // Mask password in database URL
    if let Some(at_pos) = url.rfind('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let prefix = &url[..colon_pos + 1];
            let suffix = &url[at_pos..];
            format!("{}***{}", prefix, suffix)
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    }
}
