use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Email(Secret<String>);

impl Email {
    pub fn parse(email: Secret<String>) -> Result<Self> {
        if Self::validate_email(email.expose_secret()) {
            Ok(Self(email))
        } else {
            Err(eyre!("Invalid Email"))
        }
    }

    fn validate_email(s: &str) -> bool {
        s.contains("@")
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

// TODO: Why are we leaving this intentionally blank?
impl Eq for Email {}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_for_valid_input() {
        let input = "test@test.com".to_owned();
        let secret = Secret::new(input);
        let email = Email::parse(secret);
        assert_eq!(email.is_ok(), true);
    }

    #[test]
    fn should_fail_for_missing_at_symbol() {
        let input = "test.test.com".to_owned();
        let secret = Secret::new(input);
        let email = Email::parse(secret);
        assert_eq!(email.is_err(), true);
    }
}
