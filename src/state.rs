use anyhow::{Context, Result};
use sqlx::{any::AnyPoolOptions, AnyPool};

use crate::{Config, error_ctx};

pub trait AppState: Clone + Send + Sync + 'static {
    async fn from_config(config: &Config) -> Result<Self> where Self: Sized;
    async fn db(&self) -> &AnyPool;
}

#[derive(Clone)]
pub struct DefaultState {
    db_pool: AnyPool,
}

impl AppState for DefaultState {
    async fn from_config(config: &Config) -> Result<Self> {
        // Initialise database connection pool
        let db_pool = AnyPoolOptions::new()
            .connect(&config.database_url)
            .await
            .with_context(error_ctx!("Failed to connect to database"))?;
        Ok(Self { db_pool })
    }

    async fn db(&self) -> &AnyPool {
        &self.db_pool
    }
}