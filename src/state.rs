use anyhow::{Context, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::{config::Config, error_ctx};

/// Trait for application state.
///
/// This allows for mock implementations of the state for testing purposes.
pub(crate) trait AppState: Clone + Send + Sync + 'static {
    async fn from_config(config: &Config) -> Result<Self>
    where
        Self: Sized;
    fn db(&self) -> &SqlitePool;
}

#[derive(Clone)]
pub(crate) struct DefaultState {
    db_pool: SqlitePool,
}

impl AppState for DefaultState {
    async fn from_config(config: &Config) -> Result<Self> {
        // Initialise database connection pool
        let db_pool = SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await
            .with_context(error_ctx!("Failed to connect to database"))?;
        Ok(Self { db_pool })
    }

    fn db(&self) -> &SqlitePool {
        &self.db_pool
    }
}
