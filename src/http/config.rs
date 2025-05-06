use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Config {
    /// The port to listen on.
    /// Set by the PORT environment variable.
    /// If not set, defaults to 3000.
    #[arg(env = "PORT", default_value_t = 3000)]
    pub(super) port: u16,

    /// URL of MySQL database.
    /// Set by the DATABASE_URL environment variable.
    #[arg(env = "DATABASE_URL", required = true)]
    pub(super) database_url: String,
}

#[tracing::instrument]
pub fn load() -> Result<Config> {
    let config = Config::try_parse()
        .with_context(|| {
            tracing::error!("Failed to parse environment variables");
            "Failed to parse environment variables"
        })?;
    tracing::info!("Application configuration loaded");
    tracing::debug!("{:?}", config);
    Ok(config)
}