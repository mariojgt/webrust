use tracing::{info, error, warn, debug, trace};
use serde::Serialize;
use crate::config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry, Layer};
use tracing_subscriber::fmt::Layer as FmtLayer;

pub struct Log;

impl Log {
    pub fn info<T: Serialize>(message: &str, context: Option<&T>) {
        if let Some(ctx) = context {
            let json = serde_json::to_value(ctx).unwrap_or_default();
            info!(context = ?json, "{}", message);
        } else {
            info!("{}", message);
        }
    }

    pub fn error<T: Serialize>(message: &str, context: Option<&T>) {
        if let Some(ctx) = context {
            let json = serde_json::to_value(ctx).unwrap_or_default();
            error!(context = ?json, "{}", message);
        } else {
            error!("{}", message);
        }
    }

    pub fn warning<T: Serialize>(message: &str, context: Option<&T>) {
        if let Some(ctx) = context {
            let json = serde_json::to_value(ctx).unwrap_or_default();
            warn!(context = ?json, "{}", message);
        } else {
            warn!("{}", message);
        }
    }

    pub fn debug<T: Serialize>(message: &str, context: Option<&T>) {
        if let Some(ctx) = context {
            let json = serde_json::to_value(ctx).unwrap_or_default();
            debug!(context = ?json, "{}", message);
        } else {
            debug!("{}", message);
        }
    }
}

// We will move the init logic to main.rs or a helper that returns the guard.
pub fn setup() -> Option<tracing_appender::non_blocking::WorkerGuard> {
    let config = Config::new().logging;
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    let registry = Registry::default().with(env_filter);

    match config.channel.as_str() {
        "stack" => {
            let file_appender = tracing_appender::rolling::daily(&config.dir, &config.file);
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let file_layer = FmtLayer::new()
                .with_writer(non_blocking)
                .with_ansi(false);

            let stdout_layer = FmtLayer::new()
                .with_writer(std::io::stdout);

            registry.with(stdout_layer).with(file_layer).init();
            Some(guard)
        },
        "file" | "single" => {
            let file_appender = tracing_appender::rolling::daily(&config.dir, &config.file);
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let file_layer = FmtLayer::new()
                .with_writer(non_blocking)
                .with_ansi(false);

            registry.with(file_layer).init();
            Some(guard)
        },
        _ => {
            // stdout
            let stdout_layer = FmtLayer::new()
                .with_writer(std::io::stdout);
            registry.with(stdout_layer).init();
            None
        }
    }
}
