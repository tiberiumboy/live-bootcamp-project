use crate::app_state::AppState;
use crate::routes::jwt::JWToken;
use crate::utils::auth::validate_token;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};

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
