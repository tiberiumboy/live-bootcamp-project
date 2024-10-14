use super::user::UserError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, UserError> {
        if email.contains("@") {
            Ok(Self(email.to_string()))
        } else {
            Err(UserError::InvalidEmail)
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_for_valid_input() {
        let email = Email::parse("test@test.com");
        assert_eq!(email.is_ok(), true);
    }

    #[test]
    fn should_fail_for_missing_at_symbol() {
        let email = Email::parse("test.test.com");
        assert_eq!(email.is_err(), true);
    }
}
