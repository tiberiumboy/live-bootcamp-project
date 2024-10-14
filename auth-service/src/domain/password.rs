use super::user::UserError;
use regex::Regex;

// newtype pattern in rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    // TODO - revisit closure methods and impl. this accordingly
    pub fn parse(password: &str) -> Result<Self, UserError> {
        let pattern = "^[a-zA-Z0-9]+$"; // to check and see if any special character are in the password
        let re = Regex::new(pattern).unwrap();
        if password.len() >= 8 && !re.is_match(password) {
            Ok(Self(password.to_string()))
        } else {
            Err(UserError::InvalidPassword)
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_pass_valid_input() {
        let password = Password::parse("Password123!"); // least 8 character long, numbers, and symbols included.
        assert_eq!(password.is_ok(), true);
    }

    #[test]
    fn should_fail_len_short() {
        let password = Password::parse("Passwor"); // must be 8 or more character long!
        assert_eq!(password.is_err(), true);
    }

    #[test]
    fn should_fail_missing_number_or_character() {
        let password = Password::parse("password"); // it's 8 character long, but lacks uniqueness.
        assert_eq!(password.is_err(), true);
    }
}
