use crate::domain::{email::Email, EmailClient};
use color_eyre::eyre::Result;
use secrecy::ExposeSecret;

pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    #[tracing::instrument(name = "Sending email", skip_all)]
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        // So what am I'm suppose to do here? reveal our email address?
        tracing::debug!(
            "Sending email to {}\nsubject: {}\n content:{}",
            recipient.as_ref().expose_secret(),
            subject,
            content
        );
        Ok(())
    }
}
