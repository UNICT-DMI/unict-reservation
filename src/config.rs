pub use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    /// The ID for every italian person, it's used from Smartedu as username
    pub cf: String,
    /// The password of Smartedu
    pub password: String,
    /// Driver url, an example is `http://localhost:4444` for geckodriver
    pub driver_url: String,
    /// Username of the Telegram user authorized to use the bot
    pub username: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
