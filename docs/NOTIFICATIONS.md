# Notifications System

WebRust provides a unified API for sending notifications across a variety of delivery channels, including email and database.

## Creating Notifications

To create a notification, use the `make:notification` command:

```bash
cargo run -- rune make:notification WelcomeNotification
```

This will create a new notification class in `src/notifications/welcome_notification.rs`.

## Sending Notifications

### The `Notifiable` Trait

To send notifications to a user, your `User` model should implement the `Notifiable` trait.

```rust
use crate::services::notification::Notifiable;

impl Notifiable for User {
    fn route_notification_for(&self, driver: &str) -> Option<String> {
        match driver {
            "mail" => Some(self.email.clone()),
            _ => None,
        }
    }

    fn id(&self) -> String {
        self.id.to_string()
    }
}
```

### Dispatching

You can send notifications using the `NotificationManager`.

```rust
use crate::services::notification::NotificationManager;
use crate::notifications::welcome_notification::WelcomeNotification;

let user = User::find(&db, 1).await?.unwrap();
let notification = WelcomeNotification;

// Send via configured channels
NotificationManager::send_with_db(&db, &user, &notification).await?;
```

## Database Notifications

If you use the `database` channel, notifications will be stored in the `notifications` table.

### Migration

A migration for the notifications table is included. Run:

```bash
cargo run -- rune migrate
```

### Accessing Notifications

You can query the `notifications` table to display them in your UI.

```rust
let notifications = sqlx::query("SELECT * FROM notifications WHERE notifiable_id = ? ORDER BY created_at DESC")
    .bind(user.id)
    .fetch_all(&db)
    .await?;
```
