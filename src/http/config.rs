use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Config {
    /// The port to listen on.
    /// Set by the PORT environment variable.
    /// If not set, defaults to 3000.
    #[arg(env = "PORT", default_value_t = 3000, required = true)]
    port: u16,
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