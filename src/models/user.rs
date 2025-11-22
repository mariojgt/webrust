use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use crate::orbit::Orbit;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(skip)] // Don't serialize password to JSON
    pub password: Option<String>, // Option because it might not be selected in all queries
    pub created_at: DateTime<Utc>,
}

impl Orbit for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn id(&self) -> i64 {
        self.id
    }
}

impl User {
    // Custom methods can still exist
    pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await
    }
}
