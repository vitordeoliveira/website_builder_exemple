use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pg_pool: PgPool,
}

impl AppState {
    pub async fn new(db_connection_str: &str) -> Result<Self> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(25)
            .acquire_timeout(Duration::from_secs(3))
            .connect(db_connection_str)
            .await
            .context("postgres database pool creation error")
            .expect("can't connect to database");

        sqlx::migrate!("db/migrations").run(&pg_pool).await?;
        Ok(Self { pg_pool })
    }
}
