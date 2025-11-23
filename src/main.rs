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
mod orbit;
mod config;
mod support;
pub mod cache;
pub mod database;
pub mod commands;

use clap::Parser;
use crate::cli::{Cli, Command, RuneCommand};
use crate::framework::{AppState, build_tera, build_database_manager};
use crate::routes::router;
use crate::cache::{Cache, RedisCache, FileCache, MemoryCache};
use std::process::{Command as ProcessCommand, Child};
use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::sync::Arc;

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
                    println!("🛑 Press Ctrl+C to stop");

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
                    let _guard = crate::services::log::setup();

                    // Initialize Database Manager (handles multiple connections)
                    let db_manager = build_database_manager().await;

                    // Initialize Cache
                    let cache_driver = std::env::var("CACHE_DRIVER").unwrap_or_else(|_| "file".to_string());
                    let cache: Cache = match cache_driver.as_str() {
                        "redis" => {
                            let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set when CACHE_DRIVER is redis");
                            match RedisCache::new(&redis_url).await {
                                Ok(c) => {
                                    tracing::info!("✅ Redis Cache connected");
                                    Cache::Redis(c)
                                }
                                Err(e) => {
                                    tracing::error!("❌ Failed to connect to Redis: {}", e);
                                    panic!("Failed to connect to Redis");
                                }
                            }
                        }
                        "file" => {
                            tracing::info!("✅ Using File Cache (storage/cache)");
                            Cache::File(FileCache::new("storage/cache"))
                        }
                        _ => {
                            tracing::info!("✅ Using Memory Cache");
                            Cache::Memory(MemoryCache::new())
                        }
                    };

                    let tera = build_tera()?;
                    let state = AppState::new(db_manager, tera, cache);

                    let app = router(state).await;

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
                RuneCommand::MakeMigration { name } => {
                    cli::make_migration(&name)?;
                }
                RuneCommand::Migrate => {
                    cli::run_migrations().await?;
                }
                RuneCommand::MigrateRollback => {
                    cli::rollback_migrations().await?;
                }
                RuneCommand::QueueWork { queue } => {
                    let config = crate::config::Config::new();
                    let mut queue_config = config.queue.clone();
                    queue_config.queue_name = queue;

                    // Register jobs here
                    let mut registry = crate::services::queue::JobRegistry::new();
                    registry.register::<crate::services::mail::SendEmailJob>("SendEmailJob");

                    if let Err(e) = crate::services::queue::Queue::work(&queue_config, Arc::new(registry)).await {
                        eprintln!("Queue worker failed: {}", e);
                    }
                }
                RuneCommand::ScheduleRun => {
                    println!("⏰ Starting Scheduler...");
                    let scheduler = crate::services::scheduler::Scheduler::new().await.expect("Failed to create scheduler");

                    // Example task
                    scheduler.add_async("1/10 * * * * *", || async {
                        println!("Tick! (every 10s)");
                    }).await.expect("Failed to add task");

                    scheduler.start().await.expect("Failed to start scheduler");

                    // Keep alive
                    tokio::signal::ctrl_c().await?;
                    println!("🛑 Scheduler stopped");
                }
                RuneCommand::MakeAuth => {
                    cli::make_auth()?;
                }
                RuneCommand::MakePackage { name } => {
                    cli::make_package(&name)?;
                }
                RuneCommand::MakeCommand { name } => {
                    cli::make_command(&name)?;
                }
                RuneCommand::External(args) => {
                    let command_name = args.first().expect("No command specified");
                    let registry = crate::commands::kernel();

                    if let Some(cmd) = registry.get(command_name) {
                        if let Err(e) = cmd.handle(args).await {
                            eprintln!("Command failed: {}", e);
                        }
                    } else {
                        eprintln!("Command '{}' not found.", command_name);
                        eprintln!("Available commands:");
                        for name in registry.keys() {
                            eprintln!("  - {}", name);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn spawn_server(host: &str, port: u16) -> Child {
    let mut cmd = ProcessCommand::new("cargo");
    cmd.args(&["run", "--", "rune", "serve", "--host", host, "--port", &port.to_string()]);

    // Ensure we see colors in the output
    cmd.env("CARGO_TERM_COLOR", "always");

    // Set default log level to see request logs if not set by user
    if std::env::var("RUST_LOG").is_err() {
        cmd.env("RUST_LOG", "webrust=info,tower_http=debug,info");
    }

    cmd.spawn()
        .expect("Failed to start server")
}
