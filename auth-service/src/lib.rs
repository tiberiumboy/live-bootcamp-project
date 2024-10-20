use app_state::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, Router};
use axum::serve::Serve;
use axum::Json;
use domain::error::AuthAPIError;
use routes::{hello, login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};
use std::io::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_msg) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email input"),
            AuthAPIError::InvalidPassword => (StatusCode::BAD_REQUEST, "Invalid password input"),
        };
        let body = Json(ErrorResponse {
            error: error_msg.to_owned(),
        });
        (status, body).into_response()
    }
}

pub struct Application {
    server: Serve<Router, Router>,
    pub address: SocketAddr,
}

impl Application {
    pub async fn build(app_state: AppState, socket: SocketAddr) -> Result<Self> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state);
        let listener = TcpListener::bind(socket).await?;
        let address = listener.local_addr()?; // why string?
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<()> {
        println!("Listening on {}", &self.address);
        self.server.await
    }
}
