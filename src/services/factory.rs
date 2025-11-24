/// Factory Pattern for generating test/dummy data
/// Similar to Laravel Factories

use async_trait::async_trait;
use serde_json::json;

/// Base trait for all factories
#[async_trait]
pub trait Factory: Send + Sync {
    /// Generate a single instance (returns JSON for flexibility)
    async fn make(&self) -> serde_json::Value;

    /// Create and persist the instance
    async fn create(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;

    /// Create multiple instances
    async fn create_many(&self, count: usize) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        for _ in 0..count {
            results.push(self.create().await?);
        }
        Ok(results)
    }
}

/// User Factory - generates test user data
pub struct UserFactory {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}

impl UserFactory {
    pub fn new() -> Self {
        Self {
            name: None,
            email: None,
            password: None,
            is_admin: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn admin(mut self) -> Self {
        self.is_admin = Some(true);
        self
    }

    pub fn user(mut self) -> Self {
        self.is_admin = Some(false);
        self
    }
}

impl Default for UserFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Factory for UserFactory {
    async fn make(&self) -> serde_json::Value {
        let name = self.name.clone().unwrap_or_else(|| {
            let first_names = vec!["John", "Jane", "Bob", "Alice", "Charlie"];
            let last_names = vec!["Doe", "Smith", "Johnson", "Williams", "Brown"];
            let first = first_names[rand::random::<usize>() % first_names.len()];
            let last = last_names[rand::random::<usize>() % last_names.len()];
            format!("{} {}", first, last)
        });

        let email = self.email.clone().unwrap_or_else(|| {
            format!(
                "user{}@example.com",
                rand::random::<u32>() % 10000
            )
        });

        let password = self.password.clone().unwrap_or_else(|| "password".to_string());
        let is_admin = self.is_admin.unwrap_or(false);

        json!({
            "name": name,
            "email": email,
            "password": password,
            "is_admin": is_admin,
            "created_at": chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn create(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // In real implementation, this would insert into DB
        Ok(self.make().await)
    }
}

/// Post Factory - generates test post data
pub struct PostFactory {
    pub title: Option<String>,
    pub content: Option<String>,
    pub user_id: Option<i32>,
}

impl PostFactory {
    pub fn new() -> Self {
        Self {
            title: None,
            content: None,
            user_id: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = Some(user_id);
        self
    }
}

impl Default for PostFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Factory for PostFactory {
    async fn make(&self) -> serde_json::Value {
        let title = self.title.clone().unwrap_or_else(|| {
            let titles = vec![
                "Getting Started with Rust",
                "Understanding Async/Await",
                "WebRust Framework Guide",
                "Best Practices for Web Development",
                "Advanced Database Patterns",
            ];
            titles[rand::random::<usize>() % titles.len()].to_string()
        });

        let content = self.content.clone().unwrap_or_else(|| {
            "This is a sample post content for testing and development purposes.".to_string()
        });

        let user_id = self.user_id.unwrap_or(1);

        json!({
            "title": title,
            "content": content,
            "user_id": user_id,
            "published": false,
            "created_at": chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn create(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // In real implementation, this would insert into DB
        Ok(self.make().await)
    }
}

/// Comment Factory - generates test comment data
pub struct CommentFactory {
    pub content: Option<String>,
    pub user_id: Option<i32>,
    pub post_id: Option<i32>,
}

impl CommentFactory {
    pub fn new() -> Self {
        Self {
            content: None,
            user_id: None,
            post_id: None,
        }
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_post_id(mut self, post_id: i32) -> Self {
        self.post_id = Some(post_id);
        self
    }
}

impl Default for CommentFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Factory for CommentFactory {
    async fn make(&self) -> serde_json::Value {
        let content = self.content.clone().unwrap_or_else(|| {
            let comments = vec![
                "Great post! Very helpful.",
                "I have a question about this.",
                "Thanks for sharing this knowledge.",
                "This helped me solve my problem.",
                "Can you elaborate on this point?",
            ];
            comments[rand::random::<usize>() % comments.len()].to_string()
        });

        let user_id = self.user_id.unwrap_or(1);
        let post_id = self.post_id.unwrap_or(1);

        json!({
            "content": content,
            "user_id": user_id,
            "post_id": post_id,
            "created_at": chrono::Utc::now().to_rfc3339(),
        })
    }

    async fn create(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // In real implementation, this would insert into DB
        Ok(self.make().await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_factory() {
        let factory = UserFactory::new();
        let user = factory.make().await;
        
        assert!(user.get("name").is_some());
        assert!(user.get("email").is_some());
        assert!(user.get("password").is_some());
    }

    #[tokio::test]
    async fn test_user_factory_with_builder() {
        let factory = UserFactory::new()
            .with_name("John Doe")
            .with_email("john@example.com")
            .admin();
        
        let user = factory.make().await;
        
        assert_eq!(user["name"], "John Doe");
        assert_eq!(user["email"], "john@example.com");
        assert_eq!(user["is_admin"], true);
    }

    #[tokio::test]
    async fn test_post_factory() {
        let factory = PostFactory::new().with_user_id(5);
        let post = factory.make().await;
        
        assert!(post.get("title").is_some());
        assert_eq!(post["user_id"], 5);
    }
}
