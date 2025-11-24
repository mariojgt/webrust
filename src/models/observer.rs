use async_trait::async_trait;
use serde_json::Value;

/// Observer trait - implement this to observe model lifecycle events
#[async_trait]
pub trait Observer: Send + Sync {
    /// Called when a model is being created (before save)
    async fn creating(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called after a model is created (after save)
    async fn created(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called when a model is being updated (before save)
    async fn updating(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called after a model is updated (after save)
    async fn updated(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called when a model is being deleted
    async fn deleting(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called after a model is deleted
    async fn deleted(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called when a model is being saved (create or update)
    async fn saving(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Called after a model is saved
    async fn saved(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

/// Model observable trait - add to your models
#[async_trait]
pub trait Observable: Send + Sync {
    /// Get observers for this model
    fn observers() -> Vec<Box<dyn Observer>>;

    /// Trigger creating event
    async fn fire_creating(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.creating(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger created event
    async fn fire_created(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.created(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger updating event
    async fn fire_updating(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.updating(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger updated event
    async fn fire_updated(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.updated(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger deleting event
    async fn fire_deleting(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.deleting(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger deleted event
    async fn fire_deleted(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.deleted(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger saving event
    async fn fire_saving(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.saving(&Value::Null).await?;
        }
        Ok(())
    }

    /// Trigger saved event
    async fn fire_saved(&self) -> Result<(), Box<dyn std::error::Error>> {
        for observer in Self::observers() {
            observer.saved(&Value::Null).await?;
        }
        Ok(())
    }
}

// ============================================================================
// EXAMPLE OBSERVERS
// ============================================================================

pub struct UserObserver;

#[async_trait]
impl Observer for UserObserver {
    async fn created(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("👤 User created - sending welcome email");
        // Send welcome email
        Ok(())
    }

    async fn updated(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 User updated - logging changes");
        // Log changes to audit table
        Ok(())
    }

    async fn deleted(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗑️  User deleted - cleaning up related data");
        // Delete user posts, comments, etc.
        Ok(())
    }
}

pub struct PostObserver;

#[async_trait]
impl Observer for PostObserver {
    async fn created(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 Post created - incrementing author reputation");
        // Increment author reputation
        Ok(())
    }

    async fn updated(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("✏️  Post updated - clearing cache");
        // Clear cache for this post
        Ok(())
    }

    async fn deleted(&self, _model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗑️  Post deleted - notifying followers");
        // Notify followers
        Ok(())
    }
}

pub struct AuditObserver;

#[async_trait]
impl Observer for AuditObserver {
    async fn created(&self, model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Audit: Model created: {:?}", model);
        Ok(())
    }

    async fn updated(&self, model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Audit: Model updated: {:?}", model);
        Ok(())
    }

    async fn deleted(&self, model: &Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("📋 Audit: Model deleted: {:?}", model);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModel;

    #[async_trait]
    impl Observable for TestModel {
        fn observers() -> Vec<Box<dyn Observer>> {
            vec![Box::new(UserObserver)]
        }
    }

    #[tokio::test]
    async fn test_observer_created() {
        let model = TestModel;
        let result = model.fire_created().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_observer_updated() {
        let model = TestModel;
        let result = model.fire_updated().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_observer_deleted() {
        let model = TestModel;
        let result = model.fire_deleted().await;
        assert!(result.is_ok());
    }
}
