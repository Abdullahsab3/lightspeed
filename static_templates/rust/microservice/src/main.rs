use anyhow::Result;
use log::debug;
use models::config::AppConfig;
use utils::sqlx_utils::connect_to_db;

use crate::http::serve;

pub mod controllers;
pub mod error;
pub mod http;
pub mod log_request;
mod models;
pub mod services;
mod sources;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cfg = AppConfig::init_env()?;
    let pool = connect_to_db(&cfg.DATABASE_URL, cfg.DATABASE_CONNECTION_RETRIES).await?;
    serve(pool).await?;

    Ok(())
}
