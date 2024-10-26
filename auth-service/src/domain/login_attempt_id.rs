use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        match id.parse::<Uuid>() {
            Ok(id) => Ok(Self(id.to_string())),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::LoginAttemptId;
    use uuid::Uuid;

    #[test]
    fn parse_should_succeed() {
        let uuid = Uuid::new_v4().to_string();
        assert!(LoginAttemptId::parse(uuid).is_ok());
    }

    #[test]
    fn default_should_succeed() {
        let id = LoginAttemptId::default();
        assert!(!id.as_ref().is_empty())
    }

    #[test]
    fn malform_input_should_fail() {
        let mut uuid_1 = Uuid::new_v4().to_string().to_owned();
        uuid_1.push_str("@"); // malformed character added.
        let test_case = ["test", "123", &uuid_1];
        for test in test_case {
            let response = LoginAttemptId::parse(test.to_owned());
            assert!(response.is_err());
        }
    }
}
