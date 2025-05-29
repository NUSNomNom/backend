#![forbid(unsafe_code)]
#![warn(clippy::correctness, clippy::pedantic, clippy::style, clippy::perf)]

mod app;
mod config;
mod macros;
mod models;
mod routes;
mod state;

#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Load application configuration
    let config = config::load()?;

    // Set up listening socket
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).await.with_context(|| {
        let msg = format!("Failed to bind to address {addr}");
        tracing::error!(msg);
        msg
    })?;
    info!("Listening on port {}", config.port);

    // Delegate startup to server
    app::serve(config, listener)
        .await
        .with_context(|| format!("Failed to start server on {addr}"))
}
