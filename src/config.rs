use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub cors_allowed_origins: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::Environment::with_prefix("RUST_MATH").separator("__"));
        builder.build()?.try_deserialize()
    }
}
