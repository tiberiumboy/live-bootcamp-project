use crate::services::hashmap_user_store::UserStoreError;

#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
    InvalidPassword,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, UserError> {
        if password.len() >= 8 {
            Ok(Self(password.to_string()))
        } else {
            Err(UserError::InvalidPassword)
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    email: Email,
    password: Password,
    requires_2fa: bool,
}

impl User {
    pub fn parse(email: &str, password: &str, requires_2fa: bool) -> Result<User, UserError> {
        let email = Email::parse(email)?;
        let password = Password::parse(password)?;
        Ok(User {
            email,
            password,
            requires_2fa,
        })
    }

    pub fn get_email(&self) -> &str {
        &self.email.0
    }

    pub fn password_match(&self, password: &str) -> bool {
        self.password.0 == password
    }
}
