use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(skip)] // Don't serialize password to JSON
    pub password: Option<String>, // Option because it might not be selected in all queries
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn all(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, NULL as password, created_at
            FROM users
            ORDER BY id DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await
    }
}
