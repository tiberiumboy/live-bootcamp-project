use color_eyre::eyre::{eyre, Result};
use regex::Regex;
use secrecy::{ExposeSecret, Secret};

// newtype pattern in rust
#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

impl Password {
    // TODO - revisit closure methods and impl. this accordingly
    pub fn parse(password: Secret<String>) -> Result<Self> {
        if Self::validate_password(&password) {
            Ok(Self(password))
        } else {
            Err(eyre!("Invalid Password"))
        }
    }

    fn validate_password(s: &Secret<String>) -> bool {
        let pattern = "^[a-zA-Z0-9]+$"; // to check and see if any special character are in the password
        let re = Regex::new(pattern).unwrap();
        s.expose_secret().len() >= 8 && !re.is_match(s.expose_secret())
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_pass_valid_input() {
        let input = "Password123!".to_string();
        let secret = Secret::new(input);
        let password = Password::parse(secret); // least 8 character long, numbers, and symbols included.
        assert_eq!(password.is_ok(), true);
    }

    #[test]
    fn should_fail_len_short() {
        let input = "Passwor".to_string();
        let secret = Secret::new(input);
        let password = Password::parse(secret); // must be 8 or more character long!
        assert_eq!(password.is_err(), true);
    }

    #[test]
    fn should_fail_missing_number_or_character() {
        let input = "password".to_string();
        let secret = Secret::new(input);
        let password = Password::parse(secret); // it's 8 character long, but lacks uniqueness.
        assert_eq!(password.is_err(), true);
    }
}
