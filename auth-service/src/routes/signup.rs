use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
use crate::domain::user::{User, UserError};

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
    #[serde(alias = "requires2FA")]
    requires_2fa: bool,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    message: String,
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // if we have invalid input from either email or password, return 400 for invalid input
    // create a new user template
    let user = match User::parse(&request.email, &request.password, request.requires_2fa) {
        Ok(user) => user,
        Err(e) => match e {
            UserError::InvalidEmail => return Err(AuthAPIError::InvalidEmail),
            UserError::InvalidPassword => return Err(AuthAPIError::InvalidPassword),
        },
    };

    // access database
    let mut user_store = state.user_store.write().await;

    // try adding the user.
    match user_store.add_user(user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((StatusCode::CREATED, response).into_response())
        }
        // probably a bad practice?
        Err(_) => Err(AuthAPIError::UserAlreadyExists),
    }
}
