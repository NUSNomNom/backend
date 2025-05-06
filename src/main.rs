#![forbid(unsafe_code)]
#![warn(clippy::correctness, clippy::pedantic, clippy::style, clippy::perf)]

mod http;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Load application configuration
    let config = http::config::load()?;

    // Set up listening socket
    // TODO: Read port from environment variable PORT
    let port = 3000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to address {}", addr))?;
    info!("Listening on port {}", port);

    // Delegate startup to server
    http::serve(config, listener)
        .await
        .with_context(|| format!("Failed to start server on {}", addr))
}
