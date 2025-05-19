pub mod config;
mod v1;

use anyhow::{Context, Result};
use axum::Router;
use config::Config;
use sqlx::{AnyPool, any::AnyPoolOptions};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};

#[macro_export]
macro_rules! error_ctx {
    ($($arg:tt)*) => {
        || {
            let msg = format!($($arg)*);
            tracing::error!(msg);
            msg
        }
    };
}

#[derive(Clone)]
struct AppState {
    db_pool: AnyPool,
}

impl AppState {
    async fn from_config(config: &Config) -> Result<AppState> {
        // Initialise database connection pool
        let db_pool = AnyPoolOptions::new()
            .connect(&config.database_url)
            .await
            .with_context(error_ctx!("Failed to connect to database"))?;
        Ok(Self { db_pool })
    }
}

async fn make_app(config: &Config) -> Result<Router<()>> {
    // Initialise application state
    let state = AppState::from_config(config)
        .await
        .with_context(error_ctx!("Failed to create application state"))?;

    // Double nesting here to allow for v2
    let router = Router::new().nest("/v1", v1::make_router());
    let router = Router::new()
        .nest("/api", router)
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .with_state(state);

    Ok(router)
}

#[tracing::instrument]
pub async fn serve(config: Config, listener: TcpListener) -> Result<()> {
    // Create application
    let app = make_app(&config)
        .await
        .with_context(error_ctx!("Failed to create application"))?;

    // Serve application
    axum::serve(listener, app)
        .await
        .with_context(error_ctx!("Failed to start server"))?;

    Ok(())
}
