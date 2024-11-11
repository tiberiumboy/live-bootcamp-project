use super::email::Email; // not sure why I need a super crate for this?
use color_eyre::eyre::Result;

#[async_trait::async_trait]
pub trait EmailClient: Sync + Send {
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()>;
}
