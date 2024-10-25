use app_state::AppState;
use axum::{
    http::{HeaderValue, Method},
    routing::{delete, get, post, Router},
    serve::Serve,
};
use routes::{delete_account, hello, login, logout, signup, verify_2fa, verify_token};
use std::{io::Result, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: SocketAddr,
}

impl Application {
    pub async fn build(app_state: AppState, socket: SocketAddr) -> Result<Self> {
        let allowed_origins = [
            "http://localhost:8000".parse::<HeaderValue>().unwrap(),
            "http://157.230.203.136:8000"
                .parse::<HeaderValue>()
                .unwrap(),
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .route("/delete-account", delete(delete_account))
            .with_state(app_state)
            .layer(cors);
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
