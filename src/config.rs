use anyhow::{Context, Result};
use clap::Parser;

use crate::error_ctx;

#[derive(Debug, Clone, Parser)]
pub(crate) struct Config {
    /// URL of `MySQL` database.
    /// Set by the `DATABASE_URL` environment variable.
    #[arg(env = "DATABASE_URL", required = true)]
    pub(super) database_url: String,

    /// The port to listen on.
    /// Set by the PORT environment variable.
    /// If not set, defaults to 3000.
    #[arg(env = "PORT", default_value_t = 3000)]
    pub(super) port: u16,

    /// The Base64-encoded secret key used for HMAC.
    /// Set by the `HMAC_SECRET` environment variable.
    #[arg(env = "HMAC_SECRET", required = true)]
    pub(super) hmac_secret: String,
}

#[tracing::instrument]
pub(crate) fn load() -> Result<Config> {
    let config =
        Config::try_parse().with_context(error_ctx!("Failed to parse environment variables"))?;
    tracing::info!("Application configuration loaded");
    tracing::debug!("{:?}", config);
    Ok(config)
}
