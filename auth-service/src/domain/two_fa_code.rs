use color_eyre::eyre::{eyre, Result};
use rand::{thread_rng, Rng};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TwoFACode(Secret<String>);

impl TwoFACode {
    pub fn parse(code: Secret<String>) -> Result<Self> {
        // Code must be exactly 6 character long - this may include padding 0's
        if !Self::validate(code.expose_secret()) {
            return Err(eyre!("Invalid code length!"));
        }
        Ok(Self(code))
    }

    fn validate(s: &str) -> bool {
        s.len() == 6 && s.chars().any(|v| !v.is_ascii_digit())
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut pad = "00000".to_owned();
        let mut rng = thread_rng();
        // generate the code as number then cast it into string
        let code = rng.gen_range(0..=999999).to_string();
        // append the number to the pad
        pad.push_str(&code);
        // truncate the pad off to exact 6 characters
        let secret = Secret::new(pad.split_off(pad.len() - 6));
        Self(secret)
    }
}

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for TwoFACode {}

impl AsRef<Secret<String>> for TwoFACode {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TwoFACode;
    use rstest::*;
    use secrecy::Secret;

    #[test]
    fn parse_should_pass() {
        // must be exactly 6 digit number
        let pass = "123456".to_owned();
        let secret = Secret::new(pass);
        let code = TwoFACode::parse(secret);
        assert!(code.is_ok())
    }

    #[test]
    fn empty_string_should_fail() {
        let pass = "".to_owned();
        let secret = Secret::new(pass);
        let response = TwoFACode::parse(secret);
        assert!(response.is_err());
    }

    #[test]
    fn string_contains_ascii_char_should_fail() {
        let pass = "12345A".to_owned();
        let secret = Secret::new(pass);
        let response = TwoFACode::parse(secret);
        assert!(response.is_err());
    }

    #[rstest]
    #[case("12345A")]
    #[case("1234A6")]
    #[case("123A56")]
    #[case("12A456")]
    #[case("1A3456")]
    #[case("A23456")]
    #[test]
    fn string_contains_numeric_should_fail(#[case] code: &str) {
        let secret = Secret::new(code.to_owned());
        let response = TwoFACode::parse(secret);
        assert!(response.is_err());
    }

    #[rstest]
    #[case(" 12345")]
    #[case("1 2345")]
    #[case("12 345")]
    #[case("123 45")]
    #[case("1234 5")]
    #[case("12345 ")]
    #[test]
    fn string_contains_space_should_fail(#[case] code: &str) {
        let secret = Secret::new(code.to_owned());
        let response = TwoFACode::parse(secret);
        assert!(response.is_err());
    }

    #[rstest]
    #[case("1")]
    #[case("12")]
    #[case("123")]
    #[case("1234")]
    #[case("12345")]
    #[case("1234567")]
    #[case("12345678")]
    #[case("123456789")]
    fn code_not_six_characters_limit_should_fail(#[case] code: &str) {
        let secret = Secret::new(code.to_owned());
        let response = TwoFACode::parse(secret);
        assert!(response.is_err());
    }

    #[test]
    fn invalid_input_should_fail() {
        let test_case = [
            "12345",   // not exactly 6 character long
            "12345A",  // contains ascii char
            "AAAAAA",  // non-numeric value
            "1234567", // exceeding character limit
            "123456 ", // contians space
            "12345 ",  // 6 characters long, but contians invalid non-numeric character
        ];
        for test in test_case {
            let secret = Secret::new(test.to_owned());
            let response = TwoFACode::parse(secret);
            assert!(response.is_err());
        }
    }
}
