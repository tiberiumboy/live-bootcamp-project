use app_state::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, Router};
use axum::serve::Serve;
use axum::Json;
use domain::error::AuthAPIError;
use routes::{delete_account, hello, login, logout, signup, verify_2fa, verify_token};
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
            .route("/delete-account", delete(delete_account))
            .with_state(app_state);
        let listener = TcpListener::bind(socket).await?;

        // let grpc = socket.clone();
        // grpc.set_port(50051);

        // let token = VerifyToken::default();
        // // TODO: find a way to change the socket port to something else?
        // let grpc = Server::builder()
        //     .add_service(VerifyTokenServer::new(token))
        //     .serve(grpc)
        //     .await;

        let address = listener.local_addr()?;
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<()> {
        println!("Listening on {}", &self.address);
        self.server.await
    }
}
