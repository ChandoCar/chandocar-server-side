#[cfg(test)]
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySqlPool};
use std::env;

pub async fn init_db_pool() -> Result<MySqlPool, Error> {
    #[cfg(test)]
    {
        dotenv().ok();
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(
            env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .as_str(),
        )
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db_pool() {
        assert!(init_db_pool().await.is_ok());
    }
}
