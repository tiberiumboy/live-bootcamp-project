use crate::domain::{email::Email, EmailClient};
use color_eyre::eyre::Result;
use reqwest::{Client, Url};
use secrecy::{ExposeSecret, Secret};

const MESSAGE_STREAM: &str = "outbound";
const POSTMARK_AUTH_HEADER: &str = "X-Postmark-Server-Token";

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
    message_stream: &'a str,
}

pub struct PostmarkEmailClient {
    client: Client,
    base_url: String,
    sender: Email,
    auth_token: Secret<String>,
}

impl PostmarkEmailClient {
    pub fn new(
        base_url: String,
        sender: Email,
        auth_token: Secret<String>,
        client: Client,
    ) -> Self {
        Self {
            client,
            base_url,
            sender,
            auth_token,
        }
    }
}

#[async_trait::async_trait]
impl EmailClient for PostmarkEmailClient {
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        let base = Url::parse(&self.base_url)?;
        let url = base.join("/email")?;

        let body = SendEmailRequest {
            from: self.sender.as_ref().expose_secret(),
            to: recipient.as_ref().expose_secret(),
            subject,
            html_body: &content,
            text_body: &content,
            message_stream: MESSAGE_STREAM,
        };

        let request = self
            .client
            .post(url)
            .header(POSTMARK_AUTH_HEADER, self.auth_token.expose_secret())
            .json(&body);

        request.send().await?.error_for_status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use crate::utils::constants::test;

    // use super::*;
    // use fake::{
    //     faker::{
    //         internet::en::SafeEmail,
    //         lorem::en::{Paragraph, Sentence},
    //     },
    //     Fake,
    // };
    // use wiremock::matchers::{any, header, header_exists, method, path};
    // use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    // use super::PostmarkEmailClient;

    // fn subject() -> String {
    //     Sentence(1..2).fake()
    // }

    // fn content() -> String {
    //     Paragraph(1..10).fake()
    // }

    // fn email() -> Email {
    //     Email::parse(Secret::new(SafeEmail().fake())).unwrap()
    // }

    /*
        fn email_client(base_url: String) -> PostmarkEmailClient {
            let client = Client::builder()
            .timeout(test::email_client::TIMEOUT)
            .build()
            .unwrap();
        PostmarkEmailClient::new(base_url, email(), Secret::new(Faker.fake()), client)
    }
    */
}
