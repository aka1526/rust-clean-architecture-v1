use sqlx::mysql::MySqlPool;
use std::error::Error;

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, Box<dyn Error>> {
    let pool = MySqlPool::connect(database_url).await?;
    Ok(pool)
} 