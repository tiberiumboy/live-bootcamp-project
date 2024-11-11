use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use secrecy::Secret;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
use crate::domain::user::User;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    email: String,
    password: Secret<String>,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    message: String,
}

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // if we have invalid input from either email or password, return 400 for invalid input
    // create a new user template
    let user = match User::parse(&request.email, request.password, request.requires_2fa) {
        Ok(user) => user,
        Err(e) => return Err(AuthAPIError::InvalidData(e.to_string())),
    };

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&user.as_ref()).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if let Err(e) = user_store.add_user(user).await {
        return Err(AuthAPIError::UnexpectedError(e.into()));
    };

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    Ok((StatusCode::CREATED, response).into_response())
}
