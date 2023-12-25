use std::{env, str::FromStr};

use crate::error::Error;
use crate::error::Result;
use log::{debug, warn};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct AppConfig {
    pub DATABASE_URL: String,
    pub DATABASE_CONNECTION_RETRIES: i32,
}

impl AppConfig {
    pub fn init_env() -> Result<AppConfig> {
        let app_config = match AppConfig::from_env() {
            Ok(config) => {
                debug!("Application Config: {:?}", &config);
                config
            }
            Err(e) => {
                warn!("Error: {:?}", &e);
                return Err(e);
            }
        };
        Ok(app_config)
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            DATABASE_URL: get_env("DATABASE_URL")?,
            DATABASE_CONNECTION_RETRIES: get_env_parse("DATABASE_CONNECTION_RETRIES")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissing(name))
}

#[allow(dead_code)]
fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}