mod framework;
mod route;
mod routes;
mod controllers;
mod models;
mod http;
mod requests;
mod services;
mod cli;
mod debug;
mod prelude;

use clap::Parser;
use crate::cli::{Cli, Command, RuneCommand};
use crate::framework::{AppState, build_tera, build_pool};
use crate::routes::router;
use tracing_subscriber::EnvFilter;
use std::process::{Command as ProcessCommand, Child};
use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Command::Rune { rune } => {
            match rune {
                RuneCommand::Dev { host, port } => {
                    println!("🚀 Starting WebRust Dev Server...");
                    println!("📍 Listening on http://{}:{}", host, port);
                    println!("💾 Watching for changes in src/ and templates/...");

                    let (tx, rx) = channel();
                    let mut watcher = notify::recommended_watcher(tx)?;

                    watcher.watch(std::path::Path::new("src"), RecursiveMode::Recursive)?;
                    watcher.watch(std::path::Path::new("templates"), RecursiveMode::Recursive)?;

                    let mut child = spawn_server(&host, port);

                    loop {
                        match rx.recv() {
                            Ok(event) => {
                                match event {
                                    Ok(event) => {
                                        // Simple debounce: consume all pending events
                                        let _ = rx.try_iter().count();

                                        println!("🔄 Change detected: {:?}", event.paths.first().map(|p| p.file_name().unwrap_or_default()));
                                        println!("Recompiling...");

                                        let _ = child.kill();
                                        let _ = child.wait();
                                        child = spawn_server(&host, port);
                                    }
                                    Err(e) => println!("watch error: {:?}", e),
                                }
                            }
                            Err(e) => println!("watch error: {:?}", e),
                        }
                    }
                }
                RuneCommand::Serve { host, port } => {
                    // logging
                    tracing_subscriber::fmt()
                        .with_env_filter(EnvFilter::from_default_env())
                        .init();

                    // Try to connect to database, but don't fail if it's not available
                    let pool = match build_pool().await {
                        Ok(p) => {
                            tracing::info!("✅ Database connected");
                            Some(p)
                        }
                        Err(e) => {
                            tracing::warn!("⚠️  Database connection failed: {}. Running without database.", e);
                            None
                        }
                    };

                    let tera = build_tera()?;
                    let state = AppState::new(pool, tera);

                    let app = router(state);

                    let addr = format!("{}:{}", host, port);
                    tracing::info!("🚀 WebRust running at http://{}", addr);

                    let listener = tokio::net::TcpListener::bind(&addr).await?;
                    axum::serve(listener, app).await?;
                }
                RuneCommand::Setup => {
                    cli::run_setup().await?;
                }
                RuneCommand::MakeController { name } => {
                    cli::make_controller(&name)?;
                }
                RuneCommand::MakeModel { name } => {
                    cli::make_model(&name)?;
                }
                RuneCommand::MakeMiddleware { name } => {
                    cli::make_middleware(&name)?;
                }
                RuneCommand::MakeRequest { name } => {
                    cli::make_request(&name)?;
                }
                RuneCommand::Migrate => {
                    cli::run_migrations().await?;
                }
            }
        }
    }

    Ok(())
}

fn spawn_server(host: &str, port: u16) -> Child {
    ProcessCommand::new("cargo")
        .args(&["run", "--", "rune", "serve", "--host", host, "--port", &port.to_string()])
        .spawn()
        .expect("Failed to start server")
}
