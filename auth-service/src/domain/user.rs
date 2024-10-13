#[derive(Debug, Clone)]
pub struct User {
    email: String,
    password: String,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn password_match(&self, password: &str) -> bool {
        self.password == password
    }
}
