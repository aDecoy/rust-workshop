use figment::providers::{Env, Format};
use figment::Figment;
use serde::Deserialize;

use super::core::ApplicationError;

#[derive(Deserialize)]
pub struct Config {
    database: DatabaseConfiguration,
    app_port: Option<u16>,
}

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    connection_string: String,
}

impl Config {
    pub fn get_configuration() -> Result<Self, ApplicationError> {
        let config: Config = Figment::new()
            .merge(Env::raw())
            .merge(figment::providers::Json::file("config.json"))
            .extract()
            .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;

        Ok(config)
    }

    pub fn connection_string(&self) -> String {
        self.database.connection_string.clone()
    }

    pub fn app_port(&self) -> u16 {
        self.app_port.unwrap_or(3000)
    }
}