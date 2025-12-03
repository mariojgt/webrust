use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::services::mail::{Mail, MailMessage};
use crate::config::Config;
use crate::database::DatabaseManager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseMessage {
    pub message: String,
    pub data: Value,
}

#[async_trait]
pub trait Notifiable: Send + Sync {
    /// Get the notification routing information for the given driver.
    fn route_notification_for(&self, driver: &str) -> Option<String>;

    /// Get the primary key of the notifiable entity (usually for database notifications)
    fn id(&self) -> String;

    /// Get the type of the notifiable entity (e.g., "users")
    fn notifiable_type(&self) -> String {
        "users".to_string()
    }
}

#[async_trait]
pub trait Notification: Send + Sync {
    /// Get the notification's delivery channels.
    fn via(&self, notifiable: &dyn Notifiable) -> Vec<String>;

    /// Get the mail representation of the notification.
    fn to_mail(&self, _notifiable: &dyn Notifiable) -> Option<MailMessage> {
        None
    }

    /// Get the database representation of the notification.
    fn to_database(&self, _notifiable: &dyn Notifiable) -> Option<DatabaseMessage> {
        None
    }
}

pub struct NotificationManager;

impl NotificationManager {
    pub async fn send(notifiable: &dyn Notifiable, notification: &dyn Notification) -> Result<(), String> {
        let channels = notification.via(notifiable);
        let config = Config::new();

        for channel in channels {
            match channel.as_str() {
                "mail" => {
                    if let Some(mail_message) = notification.to_mail(notifiable) {
                        if let Some(email) = notifiable.route_notification_for("mail") {
                            // We use send_now for simplicity, but in production this should probably be queued
                            // if the notification itself isn't already queued.
                            // For now, we'll assume immediate sending or let the Mail service handle queuing if configured.
                            // The Mail::send method queues it.
                            Mail::send(&config, &email, &mail_message.subject, &mail_message.body)?;
                        }
                    }
                },
                "database" => {
                    if let Some(db_message) = notification.to_database(notifiable) {
                        // We need a way to access the database here.
                        // Ideally, NotificationManager should be instantiated with state or DB access.
                        // For this static implementation, we might need to pass the DB manager or use a global/service locator pattern if available.
                        // However, since we are in a static context, we might need to change the signature of `send`.
                        // But to keep it simple for now, we will just log it if we can't access DB,
                        // OR we require the user to pass the DB manager to `send`.
                        println!("⚠️ Database notifications require a DatabaseManager instance. Use `send_with_db` instead.");
                    }
                },
                "log" => {
                     println!("🔔 [NOTIFICATION] To: {:?}, Type: {:?}", notifiable.route_notification_for("mail"), std::any::type_name::<dyn Notification>());
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub async fn send_with_db(
        db: &DatabaseManager,
        notifiable: &dyn Notifiable,
        notification: &dyn Notification
    ) -> Result<(), String> {
        let channels = notification.via(notifiable);
        let config = Config::new();

        for channel in channels {
            match channel.as_str() {
                "mail" => {
                    if let Some(mail_message) = notification.to_mail(notifiable) {
                        if let Some(email) = notifiable.route_notification_for("mail") {
                            Mail::send(&config, &email, &mail_message.subject, &mail_message.body)?;
                        }
                    }
                },
                "database" => {
                    if let Some(db_message) = notification.to_database(notifiable) {
                        let id = uuid::Uuid::new_v4().to_string();
                        let type_name = "Notification"; // In a real app, we'd want the struct name
                        let notifiable_type = notifiable.notifiable_type();
                        let notifiable_id = notifiable.id();
                        let data = serde_json::to_string(&db_message.data).unwrap_or_default();

                        // Insert into notifications table
                        // Assumes a table `notifications` exists:
                        // id (uuid), type, notifiable_type, notifiable_id, data, read_at, created_at, updated_at
                        let sql = "INSERT INTO notifications (id, type, notifiable_type, notifiable_id, data, created_at, updated_at) VALUES (?, ?, ?, ?, ?, NOW(), NOW())";

                        // Note: This requires the `notifications` table migration.
                        // We will use sqlx directly via the manager.
                        if let Some(pool) = db.connection(None) {
                             let _ = sqlx::query(sql)
                                .bind(id)
                                .bind(type_name)
                                .bind(notifiable_type)
                                .bind(notifiable_id)
                                .bind(data)
                                .execute(pool)
                                .await
                                .map_err(|e| e.to_string())?;
                        }
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
}
