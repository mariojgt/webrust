use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
pub struct DatabaseConnectionConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub default: String,
    pub connections: HashMap<String, DatabaseConnectionConfig>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let mut connections = HashMap::new();

        // Default connection (from env)
        let default_connection = env::var("DB_CONNECTION").unwrap_or_else(|_| "mysql".to_string());

        // We can add more presets here, but for now let's just map the env vars to the "default" connection
        // or strictly follow Laravel's style where we have named connections.

        // "mysql" connection
        connections.insert("mysql".to_string(), DatabaseConnectionConfig {
            url: env::var("DATABASE_URL").unwrap_or_default(),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
        });

        // "sqlite" connection example
        connections.insert("sqlite".to_string(), DatabaseConnectionConfig {
            url: env::var("DB_SQLITE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string()),
            max_connections: 1,
        });

        Self {
            default: default_connection,
            connections,
        }
    }
}
