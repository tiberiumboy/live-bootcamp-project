use serde::{Deserialize, Serialize};

// this might change...
#[derive(Debug, Serialize, Deserialize)]
pub struct JWToken {
    pub token: String,
}
