use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;
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
    fn hmac(&self) -> &Hmac<Sha256>;
}

#[derive(Clone)]
pub(crate) struct DefaultState {
    db_pool: SqlitePool,
    hmac: Hmac<Sha256>,
}

impl AppState for DefaultState {
    async fn from_config(config: &Config) -> Result<Self> {
        // Initialise database connection pool
        let db_pool = SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await
            .with_context(error_ctx!("Failed to connect to database"))?;

        // Initialise HMAC with the provided secret
        let hmac = Hmac::<Sha256>::new_from_slice(config.hmac_secret.as_bytes())
            .with_context(error_ctx!("Failed to create HMAC instance"))?;
        Ok(Self { db_pool, hmac })
    }

    fn db(&self) -> &SqlitePool {
        &self.db_pool
    }

    fn hmac(&self) -> &Hmac<Sha256> {
        &self.hmac
    }
}
