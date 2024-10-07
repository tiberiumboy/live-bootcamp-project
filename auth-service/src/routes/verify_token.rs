use crate::routes::jwt::JWToken;
use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;

pub async fn verify_token(Json(token): Json<JWToken>) -> impl IntoResponse {
    dbg!(token);
    StatusCode::OK.into_response()
}
