use anyhow::{Context, Result};
use sqlx::{any::AnyPoolOptions, AnyPool};

use crate::{config::Config, error_ctx};

/// Trait for application state.
///
/// This allows for mock implementations of the state for testing purposes.
pub(crate) trait AppState: Clone + Send + Sync + 'static {
    async fn from_config(config: &Config) -> Result<Self>
    where
        Self: Sized;
    async fn db(&self) -> &AnyPool;
}

#[derive(Clone)]
pub(crate) struct DefaultState {
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
