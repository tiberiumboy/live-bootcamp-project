use app_state::AppState;
use axum::{
    http::{HeaderValue, Method},
    routing::{get, post, Router},
    serve::Serve,
};
use redis::{Client, RedisResult};
use routes::{hello, login, logout, signup, verify_2fa, verify_token};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::io;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use utils::tracing::{make_span_with_request_id, on_request, on_response};

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
    pub async fn build(app_state: AppState, socket: SocketAddr) -> Result<Self, io::Error> {
        // TODO: Find a way to make this dynamic instead of hardcoded. - See Sidequest
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
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = TcpListener::bind(socket).await?;
        let address = listener.local_addr()?;
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new().max_connections(5).connect(url).await
    }

    pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
        let redis_url = format!("redis://{}/", redis_hostname);
        redis::Client::open(redis_url)
    }

    pub async fn run(self) -> Result<(), io::Error> {
        tracing::info!("Listening on {}", &self.address);
        self.server.await
    }
}
