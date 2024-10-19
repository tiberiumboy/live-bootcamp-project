use auth_service::app_state::AppState;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::Application;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
pub async fn main() {
    // Use ip 0.0.0.0 so service can listen on all network interfaces
    // Required for Docker to work!
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    let addr = match env::var("AUTH_SERVICE_IP") {
        Ok(addr) => addr.parse().unwrap_or(Ipv4Addr::UNSPECIFIED),
        Err(_) => Ipv4Addr::UNSPECIFIED,
    };
    let ip4 = IpAddr::V4(addr);
    let socket = SocketAddr::new(ip4, 3000);

    // our first dependency injection
    let hashmap = HashmapUserStore::default();
    let user_store = Arc::new(RwLock::new(hashmap));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, socket)
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
