# 📧 Mail

WebRust provides a clean, simple API for sending email using the `lettre` crate.

## Configuration

Mail configuration is located in `src/config/mail.rs`.

Supported drivers:
- `smtp`: Sends real emails using an SMTP server.
- `log`: Writes email details to the console (useful for local development).

### SMTP Configuration

You can configure the defaults in `src/config/mail.rs` or override them via environment variables if you implement `dotenv` loading for config.

```rust
pub struct MailConfig {
    pub driver: "smtp".to_string(),
    pub host: "smtp.mailtrap.io".to_string(),
    pub port: 2525,
    pub username: Some("user".to_string()),
    pub password: Some("pass".to_string()),
    pub from_address: "hello@example.com".to_string(),
    pub from_name: "WebRust".to_string(),
}
```

## Sending Mail

You can use the `Mail` facade (service) to send raw emails.

```rust
use crate::services::mail::Mail;

// In a controller
pub async fn send_test_email(State(state): State<AppState>) -> Html<String> {
    match Mail::send(
        &state.config.mail,
        "user@example.com",
        "Welcome!",
        "Hello from WebRust!"
    ) {
        Ok(_) => Html("Email sent!".to_string()),
        Err(e) => Html(format!("Error: {}", e)),
    }
}
```

## Mailables (Coming Soon)

In the future, WebRust will support "Mailables" – classes that build the email message, allowing you to use Tera templates for email content.
