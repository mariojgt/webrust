use async_trait::async_trait;
use serde_json::Value;

/// Policy result type
pub type PolicyResult = Result<bool, Box<dyn std::error::Error>>;

/// Authorization policy trait
#[async_trait]
pub trait Policy: Send + Sync {
    /// Check if user is authorized to view the resource
    async fn view(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        Ok(false)
    }

    /// Check if user is authorized to create a resource
    async fn create(&self, _user: &Value) -> PolicyResult {
        Ok(false)
    }

    /// Check if user is authorized to update the resource
    async fn update(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        Ok(false)
    }

    /// Check if user is authorized to delete the resource
    async fn delete(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        Ok(false)
    }

    /// Check if user is authorized to restore the resource
    async fn restore(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        Ok(false)
    }

    /// Check if user is authorized to permanently delete the resource
    async fn force_delete(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        Ok(false)
    }
}

/// Policy authorizer for managing authorization
pub struct Authorizer;

impl Authorizer {
    /// Authorize an action - returns Ok(true) if authorized
    pub async fn authorize(
        policy: &dyn Policy,
        user: &Value,
        resource: &Value,
        action: &str,
    ) -> PolicyResult {
        match action {
            "view" => policy.view(user, resource).await,
            "create" => policy.create(user).await,
            "update" => policy.update(user, resource).await,
            "delete" => policy.delete(user, resource).await,
            "restore" => policy.restore(user, resource).await,
            "force_delete" => policy.force_delete(user, resource).await,
            _ => Err("Unknown action".into()),
        }
    }

    /// Authorize and throw error if not authorized
    pub async fn authorize_or_fail(
        policy: &dyn Policy,
        user: &Value,
        resource: &Value,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match Self::authorize(policy, user, resource, action).await {
            Ok(true) => Ok(()),
            Ok(false) => Err("This action is unauthorized".into()),
            Err(e) => Err(e),
        }
    }
}

// ============================================================================
// EXAMPLE POLICIES
// ============================================================================

pub struct PostPolicy;

#[async_trait]
impl Policy for PostPolicy {
    async fn view(&self, user: &Value, post: &Value) -> PolicyResult {
        // Anyone can view posts
        Ok(true)
    }

    async fn create(&self, user: &Value) -> PolicyResult {
        // Only authenticated users can create posts
        Ok(user.get("id").is_some())
    }

    async fn update(&self, user: &Value, post: &Value) -> PolicyResult {
        // Users can only update their own posts
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let post_user_id = post.get("user_id").and_then(|v| v.as_i64());

        match (user_id, post_user_id) {
            (Some(uid), Some(pid)) => Ok(uid == pid),
            _ => Ok(false),
        }
    }

    async fn delete(&self, user: &Value, post: &Value) -> PolicyResult {
        // Users can delete their own posts, admins can delete any
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let post_user_id = post.get("user_id").and_then(|v| v.as_i64());
        let is_admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);

        match (user_id, post_user_id) {
            (Some(uid), Some(pid)) => Ok(uid == pid || is_admin),
            _ => Ok(false),
        }
    }
}

pub struct UserPolicy;

#[async_trait]
impl Policy for UserPolicy {
    async fn view(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        // Anyone can view users
        Ok(true)
    }

    async fn create(&self, user: &Value) -> PolicyResult {
        // Only admins can create users
        let is_admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(is_admin)
    }

    async fn update(&self, user: &Value, resource: &Value) -> PolicyResult {
        // Users can update themselves, admins can update anyone
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let resource_id = resource.get("id").and_then(|v| v.as_i64());
        let is_admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);

        match (user_id, resource_id) {
            (Some(uid), Some(rid)) => Ok(uid == rid || is_admin),
            _ => Ok(false),
        }
    }

    async fn delete(&self, user: &Value, _resource: &Value) -> PolicyResult {
        // Only admins can delete users
        let is_admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(is_admin)
    }
}

pub struct CommentPolicy;

#[async_trait]
impl Policy for CommentPolicy {
    async fn view(&self, _user: &Value, _resource: &Value) -> PolicyResult {
        Ok(true)
    }

    async fn create(&self, user: &Value) -> PolicyResult {
        // Only authenticated users can comment
        Ok(user.get("id").is_some())
    }

    async fn update(&self, user: &Value, comment: &Value) -> PolicyResult {
        // Users can only update their own comments
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let comment_user_id = comment.get("user_id").and_then(|v| v.as_i64());

        match (user_id, comment_user_id) {
            (Some(uid), Some(cid)) => Ok(uid == cid),
            _ => Ok(false),
        }
    }

    async fn delete(&self, user: &Value, comment: &Value) -> PolicyResult {
        // Users can delete their own comments, admins can delete any
        let user_id = user.get("id").and_then(|v| v.as_i64());
        let comment_user_id = comment.get("user_id").and_then(|v| v.as_i64());
        let is_admin = user.get("is_admin").and_then(|v| v.as_bool()).unwrap_or(false);

        match (user_id, comment_user_id) {
            (Some(uid), Some(cid)) => Ok(uid == cid || is_admin),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_post_policy_view() {
        let policy = PostPolicy;
        let user = json!({"id": 1});
        let post = json!({"id": 1, "user_id": 2});

        let result = policy.view(&user, &post).await;
        assert!(result.unwrap()); // Anyone can view
    }

    #[tokio::test]
    async fn test_post_policy_update_own() {
        let policy = PostPolicy;
        let user = json!({"id": 1});
        let post = json!({"id": 1, "user_id": 1});

        let result = policy.update(&user, &post).await;
        assert!(result.unwrap()); // Can update own post
    }

    #[tokio::test]
    async fn test_post_policy_update_other() {
        let policy = PostPolicy;
        let user = json!({"id": 1});
        let post = json!({"id": 1, "user_id": 2});

        let result = policy.update(&user, &post).await;
        assert!(!result.unwrap()); // Cannot update other's post
    }

    #[tokio::test]
    async fn test_post_policy_admin_delete() {
        let policy = PostPolicy;
        let admin = json!({"id": 1, "is_admin": true});
        let post = json!({"id": 1, "user_id": 2});

        let result = policy.delete(&admin, &post).await;
        assert!(result.unwrap()); // Admin can delete any post
    }

    #[tokio::test]
    async fn test_user_policy_create_admin_only() {
        let policy = UserPolicy;
        let regular_user = json!({"id": 1, "is_admin": false});
        let admin = json!({"id": 2, "is_admin": true});

        let regular_result = policy.create(&regular_user).await;
        let admin_result = policy.create(&admin).await;

        assert!(!regular_result.unwrap()); // Regular user cannot create
        assert!(admin_result.unwrap()); // Admin can create
    }

    #[tokio::test]
    async fn test_authorizer() {
        let policy = PostPolicy;
        let user = json!({"id": 1});
        let post = json!({"id": 1, "user_id": 1});

        let result = Authorizer::authorize(&policy, &user, &post, "update").await;
        assert!(result.unwrap()); // Should be authorized
    }
}
