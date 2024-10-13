use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::user::User;

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
) -> impl IntoResponse {
    // TODO: before we do this, we need to validate our user inputs.
    // if we have invalid input from either email or password, return 400 for invalid input

    // create a new user template
    let user = User::new(
        request.email.clone(),
        request.password,
        request.requires_2fa,
    );

    // access database
    let mut user_store = state.user_store.write().await;

    // try adding the user.
    match user_store.add_user(user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            (StatusCode::CREATED, response)
        }
        Err(_) => {
            let response = Json(SignupResponse {
                message: "Email already exists".to_owned(),
            });
            (StatusCode::CONFLICT, response)
        }
    }
}
