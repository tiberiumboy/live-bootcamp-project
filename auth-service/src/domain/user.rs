use super::{email::Email, password::Password};
use color_eyre::eyre::Result;
use secrecy::Secret;

// #[derive(Debug, Clone, Default)]
// pub enum UserRole {
//     #[default]
//     None = 0,
//     Guest = 1,
//     Member = 2,
//     Admin = 3,
// }

#[derive(Debug, Clone)]
pub struct User {
    email: Email,
    password: Password,
    requires_2fa: bool,
    // user_role: UserRole,
}

impl User {
    // TODO: Talk about this?
    pub(crate) fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
            // user_role: UserRole::default(),
        }
    }

    pub fn parse(
        email: Secret<String>,
        password: Secret<String>,
        requires_2fa: bool,
    ) -> Result<User> {
        let email = Email::parse(email)?;
        let password = Password::parse(password)?;
        Ok(User {
            email,
            password,
            requires_2fa,
            // user_role: UserRole::default(),
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
