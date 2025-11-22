use sqlx::Pool;

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
