use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Code must be exactly 6 character long - this may include padding 0's
        if code.len() != 6 {
            return Err("Invalid code length!".to_owned());
        }

        // Validate string input must be numeric only
        if code.parse::<u32>().is_err() {
            return Err("Code contains non-digit value!".to_owned());
        }

        Ok(Self(code))
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
        Self(pad.split_off(pad.len() - 6))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TwoFACode;

    #[test]
    fn parse_should_pass() {
        // must be exactly 6 digit number
        let pass = "123456".to_owned();
        let code = TwoFACode::parse(pass);
        assert!(code.is_ok())
    }

    #[test]
    fn default_should_pass() {
        let code = TwoFACode::default();
        assert!(code.as_ref().len() == 6);
        assert!(code.as_ref().chars().all(|v| v.is_ascii_digit()))
    }

    #[test]
    fn invalid_input_should_fail() {
        let test_case = [
            "12345",   // not exactly 6 character long
            "12345A",  // contains ascii char
            "AAAAAA",  // non-numeric value
            "1234567", // exceeding character limit
            "",        // empty
            "123456 ", // contians space
            "12345 ",  // 6 characters long, but contians invalid non-numeric character
        ];
        for test in test_case {
            let response = TwoFACode::parse(test.to_owned());
            assert!(response.is_err());
        }
    }
}
