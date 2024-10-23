use crate::routes::jwt::JWToken;
use crate::utils::auth::validate_token;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};

pub async fn verify_token(Json(jwt): Json<JWToken>) -> impl IntoResponse {
    match validate_token(&jwt.token).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
