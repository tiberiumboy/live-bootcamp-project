use crate::routes::jwt::JWToken;
use axum::{response::IntoResponse, Json};
use jsonwebtoken::{encode, Algorithm, Validation};
use reqwest::StatusCode;

tonic::include_proto!("verify_token_service");

// this is used for HTTP/HTTPs request from axum
pub async fn verify_token(Json(token): Json<JWToken>) -> impl IntoResponse {
    dbg!(token);
    StatusCode::OK.into_response()
}

#[derive(Debug, Default)]
pub struct VerifyTokenService {}

// TODO impl. gRPC for IPC between auth and app service
