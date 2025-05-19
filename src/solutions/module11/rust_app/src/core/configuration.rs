use figment::providers::{Env, Format};
use figment::Figment;
use serde::Deserialize;

use super::core::ApplicationError;

#[derive(Deserialize)]
pub struct Config {
    database: DatabaseConfiguration,
    messaging: Option<KafkaConfiguration>,
    app_port: Option<u16>,
}

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    connection_string: String,
}

#[derive(Deserialize)]
pub struct KafkaConfiguration {
    broker: String,
    username: Option<String>,
    password: Option<String>,
    group_id: String,
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

    pub fn kafka_broker(&self) -> String {
        self.messaging
            .as_ref()
            .map(|kafka| kafka.broker.clone())
            .unwrap_or_else(|| "localhost:9092".to_string())
    }
    pub fn kafka_username(&self) -> Option<String> {
        self.messaging
            .as_ref()
            .and_then(|kafka| kafka.username.clone())
    }
    pub fn kafka_password(&self) -> Option<String> {
        self.messaging
            .as_ref()
            .and_then(|kafka| kafka.password.clone())
    }
    pub fn kafka_group_id(&self) -> String {
        self.messaging
            .as_ref()
            .map(|kafka| kafka.group_id.clone())
            .unwrap_or_else(|| "default_group".to_string())
    }

    pub fn app_port(&self) -> u16 {
        self.app_port.unwrap_or(3000)
    }
}