use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use serde_json::{json, Value};

/// Event trait - implement this for your events
#[async_trait]
pub trait Event: Send + Sync {
    /// Event name for identification
    fn name(&self) -> &'static str;
    
    /// Convert event to JSON for serialization
    fn to_json(&self) -> Value {
        json!({})
    }
}

/// Listener trait - implement this to handle events
#[async_trait]
pub trait Listener: Send + Sync {
    /// Handle the event
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>>;
}

/// Global event dispatcher
pub struct EventDispatcher {
    listeners: Arc<RwLock<HashMap<String, Vec<Arc<dyn Listener>>>>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a listener for an event
    pub async fn listen<L: Listener + 'static>(&self, event_name: &str, listener: L) {
        let mut listeners = self.listeners.write().await;
        listeners
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(Arc::new(listener));
    }

    /// Emit an event to all registered listeners
    pub async fn emit(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        let listeners = self.listeners.read().await;
        
        if let Some(event_listeners) = listeners.get(event.name()) {
            for listener in event_listeners {
                listener.handle(event).await?;
            }
        }
        
        Ok(())
    }

    /// Get number of listeners for an event
    pub async fn listener_count(&self, event_name: &str) -> usize {
        let listeners = self.listeners.read().await;
        listeners.get(event_name).map(|v| v.len()).unwrap_or(0)
    }

    /// Clear all listeners
    pub async fn clear(&self) {
        let mut listeners = self.listeners.write().await;
        listeners.clear();
    }

    /// Clear listeners for a specific event
    pub async fn clear_event(&self, event_name: &str) {
        let mut listeners = self.listeners.write().await;
        listeners.remove(event_name);
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        Self {
            listeners: Arc::clone(&self.listeners),
        }
    }
}

// ============================================================================
// EXAMPLE EVENTS
// ============================================================================

#[derive(Clone, Debug)]
pub struct UserCreatedEvent {
    pub user_id: i64,
    pub email: String,
    pub name: String,
}

#[async_trait]
impl Event for UserCreatedEvent {
    fn name(&self) -> &'static str {
        "user.created"
    }

    fn to_json(&self) -> Value {
        json!({
            "user_id": self.user_id,
            "email": self.email,
            "name": self.name,
        })
    }
}

#[derive(Clone, Debug)]
pub struct UserDeletedEvent {
    pub user_id: i64,
    pub email: String,
}

#[async_trait]
impl Event for UserDeletedEvent {
    fn name(&self) -> &'static str {
        "user.deleted"
    }

    fn to_json(&self) -> Value {
        json!({
            "user_id": self.user_id,
            "email": self.email,
        })
    }
}

#[derive(Clone, Debug)]
pub struct PostCreatedEvent {
    pub post_id: i64,
    pub user_id: i64,
    pub title: String,
}

#[async_trait]
impl Event for PostCreatedEvent {
    fn name(&self) -> &'static str {
        "post.created"
    }

    fn to_json(&self) -> Value {
        json!({
            "post_id": self.post_id,
            "user_id": self.user_id,
            "title": self.title,
        })
    }
}

// ============================================================================
// EXAMPLE LISTENERS
// ============================================================================

pub struct SendWelcomeEmailListener;

#[async_trait]
impl Listener for SendWelcomeEmailListener {
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        if event.name() == "user.created" {
            // In real implementation, send email here
            println!("📧 Sending welcome email for event: {}", event.name());
            // mail::send_welcome_email(event).await?;
        }
        Ok(())
    }
}

pub struct LogEventListener;

#[async_trait]
impl Listener for LogEventListener {
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 Event triggered: {} with data: {}", event.name(), event.to_json());
        Ok(())
    }
}

pub struct IncrementReputationListener;

#[async_trait]
impl Listener for IncrementReputationListener {
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        if event.name() == "post.created" {
            println!("⭐ Incrementing user reputation for new post");
            // db.increment_user_reputation(event.user_id).await?;
        }
        Ok(())
    }
}

pub struct NotifySubscribersListener;

#[async_trait]
impl Listener for NotifySubscribersListener {
    async fn handle(&self, event: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        if event.name() == "post.created" {
            println!("🔔 Notifying subscribers of new post");
            // notify_subscribers(event).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_emission() {
        let dispatcher = EventDispatcher::new();
        
        let event = UserCreatedEvent {
            user_id: 1,
            email: "user@example.com".to_string(),
            name: "John Doe".to_string(),
        };

        dispatcher.listen("user.created", LogEventListener).await;
        let result = dispatcher.emit(&event).await;
        
        assert!(result.is_ok());
        assert_eq!(dispatcher.listener_count("user.created").await, 1);
    }

    #[tokio::test]
    async fn test_multiple_listeners() {
        let dispatcher = EventDispatcher::new();
        
        let event = UserCreatedEvent {
            user_id: 1,
            email: "user@example.com".to_string(),
            name: "John Doe".to_string(),
        };

        dispatcher.listen("user.created", SendWelcomeEmailListener).await;
        dispatcher.listen("user.created", LogEventListener).await;
        
        assert_eq!(dispatcher.listener_count("user.created").await, 2);
        
        let result = dispatcher.emit(&event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clear_listeners() {
        let dispatcher = EventDispatcher::new();
        
        dispatcher.listen("user.created", LogEventListener).await;
        assert_eq!(dispatcher.listener_count("user.created").await, 1);
        
        dispatcher.clear_event("user.created").await;
        assert_eq!(dispatcher.listener_count("user.created").await, 0);
    }

    #[tokio::test]
    async fn test_event_to_json() {
        let event = UserCreatedEvent {
            user_id: 1,
            email: "user@example.com".to_string(),
            name: "John Doe".to_string(),
        };

        let json = event.to_json();
        assert_eq!(json["user_id"], 1);
        assert_eq!(json["email"], "user@example.com");
    }
}
