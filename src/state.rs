use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::{config::Config, error_ctx};

#[derive(Clone)]
pub(crate) struct AppState {
    db_pool: SqlitePool,
    hmac: Hmac<Sha256>,
}

impl AppState {
    pub async fn from_config(config: &Config) -> Result<Self> {
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

    pub fn db(&self) -> &SqlitePool {
        &self.db_pool
    }

    pub fn hmac(&self) -> &Hmac<Sha256> {
        &self.hmac
    }
}
