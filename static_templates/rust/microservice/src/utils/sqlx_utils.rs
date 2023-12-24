use log::error;
use sqlx::PgPool;

use crate::error::Error;
/**
 * sqlx does not retry connection to database if it refused connection the first time.
 */
pub async fn connect_to_db(database_url: &str, retries: i32) -> Result<PgPool, Error> {
    for i in 0..retries {
        match sqlx::postgres::PgPool::connect(database_url).await {
            Ok(pool) => return Ok(pool),
            Err(e) => {
                error!("Failed to connect to database: {}", e);
                if i == retries - 1 {
                    return Err(Error::DatabaseConnectionError(format!(
                        "Failed to connect to database after {} retries. Error: {}",
                        retries,
                        e
                    )));
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Err(Error::DatabaseConnectionError(format!(
        "Failed to connect to database after {} retries",
        retries
    )))

}