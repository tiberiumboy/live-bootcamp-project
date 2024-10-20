use super::{email::Email, password::Password};

#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
    InvalidPassword,
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

    pub fn password_match(&self, password: &Password) -> bool {
        self.password.eq(password)
    }

    pub fn requires_2fa(&self) -> bool {
        self.requires_2fa
    }
}

impl AsRef<Email> for User {
    fn as_ref(&self) -> &Email {
        &self.email
    }
}

impl AsRef<Password> for User {
    fn as_ref(&self) -> &Password {
        &self.password
    }
}
