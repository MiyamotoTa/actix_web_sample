use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ServerConfig {
    pub(crate) host: String,
    pub(crate) port: i32,
}

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) database_url: String,
}

impl Config {
    pub(crate) fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
