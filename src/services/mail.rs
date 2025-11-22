use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use crate::config::mail::MailConfig;

#[derive(Debug)]
pub struct MailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub struct Mail;

impl Mail {
    /// Send a raw email
    pub fn send(config: &MailConfig, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let email = Message::builder()
            .from(format!("{} <{}>", config.from_name, config.from_address).parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
            .to(format!("<{}>", to).parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| e.to_string())?;

        match config.driver.as_str() {
            "smtp" => {
                let mut builder = SmtpTransport::builder_dangerous(&config.host)
                    .port(config.port);

                if let (Some(user), Some(pass)) = (&config.username, &config.password) {
                    let creds = Credentials::new(user.clone(), pass.clone());
                    builder = builder.credentials(creds);
                }

                let mailer = builder.build();
                match mailer.send(&email) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Could not send email: {}", e)),
                }
            },
            "log" | _ => {
                println!("📧 [MAIL LOG] To: {}, Subject: {}", to, subject);
                println!("   Body: {}", body);
                Ok(())
            }
        }
    }
}
