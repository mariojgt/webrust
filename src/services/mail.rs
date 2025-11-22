use lettre::{Message, AsyncSmtpTransport, AsyncTransport, Tokio1Executor, transport::smtp::authentication::Credentials};
use crate::config::mail::MailConfig;
use crate::services::queue::{Job, Queue};
use crate::config::Config;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SendEmailJob {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[async_trait]
impl Job for SendEmailJob {
    fn name(&self) -> String {
        "SendEmailJob".to_string()
    }

    async fn handle(&self) -> Result<(), String> {
        let config = Config::new().mail;
        Mail::send_now(&config, &self.to, &self.subject, &self.body).await
    }
}

pub struct Mail;

impl Mail {
    /// Dispatch an email to the queue
    pub fn send(config: &Config, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let job = SendEmailJob {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
        };

        Queue::dispatch(&config.queue, job)
    }

    /// Send a raw email immediately (async)
    pub async fn send_now(config: &MailConfig, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let email = Message::builder()
            .from(format!("{} <{}>", config.from_name, config.from_address).parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
            .to(format!("<{}>", to).parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| e.to_string())?;

        match config.driver.as_str() {
            "smtp" => {
                let mut builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
                    .port(config.port);

                if let (Some(user), Some(pass)) = (&config.username, &config.password) {
                    let creds = Credentials::new(user.clone(), pass.clone());
                    builder = builder.credentials(creds);
                }

                let mailer = builder.build();
                match mailer.send(email).await {
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
