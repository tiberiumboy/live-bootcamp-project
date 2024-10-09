use axum::response::{Html, IntoResponse};

pub async fn hello() -> impl IntoResponse {
    let content = "<h2>Hello, Rustaceans!</h2>";
    Html(content)
}
