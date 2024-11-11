use crate::app_state::AppState;
use crate::routes::jwt::JWToken;
use crate::utils::auth::validate_token;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};

#[tracing::instrument(name = "Verify Token", skip_all)]
pub async fn verify_token(
    State(app): State<AppState>,
    Json(jwt): Json<JWToken>,
) -> impl IntoResponse {
    let store = app.banned_token_store.clone();
    let ban_list = store.read().await;
    if ban_list.check_token(&jwt.token).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match validate_token(&jwt.token).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
