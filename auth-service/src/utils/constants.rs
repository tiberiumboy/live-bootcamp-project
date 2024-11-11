use dotenvy::dotenv;
use secrecy::Secret;
use std::{env as std_env, sync::LazyLock};

pub static JWT_SECRET: LazyLock<Secret<String>> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set!");
    if secret.is_empty() {
        panic!("JWT_SECRET_ENV must not be empty!");
    }
    Secret::new(secret)
});

pub static DATABASE_URL: LazyLock<Secret<String>> = LazyLock::new(|| {
    dotenv().ok();
    let url = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set!");
    if url.is_empty() {
        panic!("DATABASE_URL must not be empty?"); // should we be allow to panic at this stage?
    }
    Secret::new(url)
});

pub static REDIS_HOST_NAME: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    std_env::var(env::REDIS_HOST_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned())
});

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const TOKEN_TTL_SECONDS: i64 = 600; // 10 minutes
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const REDIS_HOST_ENV_VAR: &str = "REDIS_HOST_NAME";
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
