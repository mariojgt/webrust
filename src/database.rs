use sqlx::Pool;
use std::collections::HashMap;
use std::sync::Arc;

pub mod migrator;

#[cfg(feature = "mysql")]
pub type Db = sqlx::MySql;
#[cfg(feature = "mysql")]
pub type DbArguments = sqlx::mysql::MySqlArguments;
#[cfg(feature = "mysql")]
pub type DbPoolOptions = sqlx::mysql::MySqlPoolOptions;

#[cfg(feature = "postgres")]
pub type Db = sqlx::Postgres;
#[cfg(feature = "postgres")]
pub type DbArguments = sqlx::postgres::PgArguments;
#[cfg(feature = "postgres")]
pub type DbPoolOptions = sqlx::postgres::PgPoolOptions;

#[cfg(feature = "sqlite")]
pub type Db = sqlx::Sqlite;
#[cfg(feature = "sqlite")]
pub type DbArguments<'q> = sqlx::sqlite::SqliteArguments<'q>;
#[cfg(feature = "sqlite")]
pub type DbPoolOptions = sqlx::sqlite::SqlitePoolOptions;

pub type DbPool = Pool<Db>;
pub type DbRow = <Db as sqlx::Database>::Row;

#[derive(Clone)]
pub struct DatabaseManager {
    default_name: String,
    pools: HashMap<String, DbPool>,
}

impl DatabaseManager {
    pub fn new(default_name: String) -> Self {
        Self {
            default_name,
            pools: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, pool: DbPool) {
        self.pools.insert(name.to_string(), pool);
    }

    pub fn connection(&self, name: Option<&str>) -> Option<&DbPool> {
        let name = name.unwrap_or(&self.default_name);
        self.pools.get(name)
    }

    /// Get the default connection
    pub fn default_connection(&self) -> Option<&DbPool> {
        self.connection(None)
    }
}
