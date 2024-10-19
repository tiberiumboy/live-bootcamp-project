use axum::response::{IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// pub mod validate_token {
//     tonic::include_proto!("validate_token");
// }

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VerifyToken {
    email: String,
    login_attempt_id: String,
    // This must be a string to includes leading zero's.
    // We will enforce the value numerical only
    #[serde(alias = "2FACode")]
    code: String,
}

impl VerifyToken {
    pub fn new(email: String, code: String) -> Self {
        Self {
            email,
            login_attempt_id: Uuid::new_v4().to_string(),
            code,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    token: String,
}

pub async fn verify_2fa(Json(input): Json<VerifyToken>) -> axum::response::Response {
    /*
        allowed exception list:
        400: Invalid Input
        401: Authentication failed
        422: Unprocessable content
        500: Unexpected error (Should never happen)
    */

    let secret_passphrase = "Let's get rusty";
    let key = EncodingKey::from_secret(secret_passphrase.as_ref());
    let token = encode(&Header::default(), &input.email, &key).unwrap();
    // let token = JWToken::validate(input.email, &input.login_attempt_id, &input.code).unwrap();
    // dbg!(token);
    let body = VerifyTokenResponse { token };
    // pray that this still works?
    let content = serde_json::to_string(&body).unwrap();
    (StatusCode::OK, content).into_response()
}
