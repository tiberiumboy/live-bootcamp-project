/*
    Two considerations:
    Once an account has been created, should the users be able to delete their account, or should this feature only be available for admin
    If user should be able to delete an account they created, where and how should this option be present?
*/

use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{email::Email, error::AuthAPIError},
};

#[derive(Debug, Deserialize)]
pub struct DeleteAccountRequest {
    email: String,
}

pub async fn delete_account(
    State(state): State<AppState>,
    Json(request): Json<DeleteAccountRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // TODO: verify through JWT to ensure current active user can delete accounts.
    // first find the email account associated with the user.
    let email = Email::parse(&request.email).map_err(|_| return AuthAPIError::InvalidEmail)?;

    let mut store = state.user_store.write().await;

    let user = store
        .get_user(&email)
        .await
        .map_err(|_| return AuthAPIError::NotFound)?;

    store.delete_user(user).await;
    Ok(StatusCode::OK.into_response())
}
