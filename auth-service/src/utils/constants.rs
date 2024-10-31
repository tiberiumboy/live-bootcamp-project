use dotenvy::dotenv;
use std::{env as std_env, sync::LazyLock};

pub static JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set!");
    if secret.is_empty() {
        panic!("JWT_SECRET_ENV must not be empty!");
    }
    secret
});

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const TOKEN_TTL_SECONDS: i64 = 600; // 10 minutes

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub mod prod {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    // Use ip 0.0.0.0 so service can listen on all network interfaces
    // Required for Docker to work!
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    const PORT: u16 = 3000;
    pub const APP_ADDR: SocketAddr = SocketAddr::new(IP_ADDR, PORT);
}

pub mod test {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
    const PORT: u16 = 0;
    pub const APP_ADDR: SocketAddr = SocketAddr::new(IP_ADDR, PORT);
}
