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
