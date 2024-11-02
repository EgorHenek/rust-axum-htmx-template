use config::{Config, ConfigError};
use serde::Deserialize;

use crate::leak_alloc;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: usize,
    pub metrics_port: usize,
    pub db_url: String,
}

pub fn load_config() -> Result<&'static AppConfig, ConfigError> {
    let config = Config::builder()
        .set_default("host", "127.0.0.1")?
        .set_default("port", "3000")?
        .set_default("metrics_port", "3001")?
        .set_default("db_url", "sqlite:./db.sqlite")?
        .add_source(config::Environment::default())
        .build()?;
    let config = config.try_deserialize::<AppConfig>()?;

    Ok(leak_alloc(config))
}
