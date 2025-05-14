pub mod config;

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

fn make_router() -> Router<AppState> {
    Router::new()
}

#[tracing::instrument]
pub async fn serve(config: Config, listener: TcpListener) -> Result<()> {
    // Initialise database connection pool
    let db_pool = AnyPoolOptions::new()
        .connect(&config.database_url)
        .await
        .with_context(error_ctx!("Failed to connect to database"))?;

    // Initialise application state
    let app_state = AppState {
        db_pool: db_pool.clone(),
};

    // Initialise router
    let router = make_router()
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
        )
        .with_state(app_state);
    
// Serve application
    axum::serve(listener, router)
        .await
.with_context(error_ctx!("Failed to start server"))?;

    // Clean up
    // Close database connection pool
    db_pool.close().await;

    Ok(())
}