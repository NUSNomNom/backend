pub mod config;

use anyhow::{Context, Result};
use axum::Router;
use config::Config;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};

#[derive(Clone)]
struct AppState {
    db_pool: MySqlPool,
}

fn make_router() -> Router<AppState> {
    Router::new()
}

#[tracing::instrument]
pub async fn serve(config: Config, listener: TcpListener) -> Result<()> {
    // Initialise database connection pool
    let db_pool = MySqlPoolOptions::new()
        .connect(&config.database_url)
        .await
        .context("Failed to connect to database")?;

    // Initialise application state
    let app_state = AppState { db_pool };

    // Initialise router
    let router = make_router()
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
        )
        .with_state(app_state);
    
    axum::serve(listener, router)
        .await
        .context("Failed to start server")?;

    Ok(())
}