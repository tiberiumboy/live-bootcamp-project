use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn logout() -> impl IntoResponse {
    println!("Logging out!");
    StatusCode::OK.into_response()
}
