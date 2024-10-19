use crate::routes::jwt::JWToken;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
// use jsonwebtoken::{encode, Algorithm, Validation};
// use reqwest::StatusCode;
// use tonic::{Request, Response, Status};
// use validatetoken::{ValidateToken, VerifyTokenProto};

// use super::VerifyTokenResponse;

// this is used for HTTP/HTTPs request from axum
pub async fn verify_token(Json(token): Json<JWToken>) -> impl IntoResponse {
    dbg!(token);
    StatusCode::OK.into_response()
}

// TODO impl. gRPC for IPC between auth and app service
#[derive(Debug, Default)]
pub struct VerifyTokenProto {}

// #[tonic::async_trait]
// impl ValidateToken for VerifyTokenProto {
//     async fn verify_token(
//         &self,
//         request: Request<VerifyTokenRequest>,
//     ) -> Result<Response<VerifyTokenResponse>, Status> {
//         todo!("not sure what this one is suppose to do - investigate?");
//         // Ok(Response)
//     }
// }
