pub use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cf: String,
    pub password: String,
    pub driver_url: String,
    pub username: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
