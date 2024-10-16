use app_state::AppState;
use axum::routing::{delete, get, post, Router};
use axum::serve::Serve;
use routes::{delete_account, hello, login, logout, signup, verify_2fa, verify_token};
use tonic::transport::Server;
use std::io::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

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

        let grpc = Server::builder().add_service(VerifyToken)

        let address = listener.local_addr()?;
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<()> {
        println!("Listening on {}", &self.address);
        self.server.await
    }
}
