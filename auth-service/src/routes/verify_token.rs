use crate::routes::jwt::JWToken;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};

pub async fn verify_token(Json(token): Json<JWToken>) -> impl IntoResponse {
    dbg!(token);
    StatusCode::OK.into_response()
}
