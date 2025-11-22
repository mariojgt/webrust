use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn all(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, created_at
            FROM users
            ORDER BY id DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}
