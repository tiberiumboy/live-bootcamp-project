use axum::Error;
use serde::{Deserialize, Serialize};

// this might change...
#[derive(Debug, Serialize, Deserialize)]
pub struct JWToken {
    token: String,
}

impl JWToken {
    pub fn validate(email: String, id: &str, code: &str) -> Result<Self, Error> {
        dbg!(email, id, code);
        // TODO: complete the token validation
        Ok(JWToken {
            token: "".to_owned(),
        })
    }
}
