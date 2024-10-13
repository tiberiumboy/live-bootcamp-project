use app_state::AppState;
use axum::routing::{get, post, Router};
use axum::serve::Serve;
use routes::{hello, login, logout, signup, verify_2fa, verify_token};
use std::io::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
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
    pub async fn build(app_state: Arc<AppState>, socket: SocketAddr) -> Result<Self> {
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
