use axum::response::IntoResponse;
use reqwest::StatusCode;

pub async fn logout() -> impl IntoResponse {
    println!("Logging out!");
    StatusCode::OK.into_response()
}
