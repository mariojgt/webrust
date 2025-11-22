use tower_sessions::Session;
use crate::models::user::User;
use sqlx::MySqlPool;

pub const AUTH_SESSION_KEY: &str = "user_id";

pub struct Auth;

impl Auth {
    /// Attempt to authenticate a user
    pub async fn attempt(
        pool: &MySqlPool,
        session: &Session,
        email: &str,
        password: &str
    ) -> Result<bool, sqlx::Error> {
        // Find user by email
        let user = User::find_by_email(pool, email).await?;

        if let Some(user) = user {
            if let Some(hash) = &user.password {
                if crate::services::hash::check(password, hash) {
                    session.insert(AUTH_SESSION_KEY, user.id).await.unwrap();
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Log the current user out
    pub async fn logout(session: &Session) {
        session.remove::<i64>(AUTH_SESSION_KEY).await.unwrap();
    }

    /// Get the current user ID
    pub async fn id(session: &Session) -> Option<i64> {
        session.get(AUTH_SESSION_KEY).await.unwrap()
    }

    /// Check if user is logged in
    pub async fn check(session: &Session) -> bool {
        Self::id(session).await.is_some()
    }
}
