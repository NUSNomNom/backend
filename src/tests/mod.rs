use anyhow::{Context, Result};
use axum::Router;
use crate::{config::Config, error_ctx, routes, state::AppState};

mod user;

async fn make_test_app() -> Result<Router<()>> {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        port: 42069,
        hmac_secret: "test_secret_key".to_string(),
    };

    let state = AppState::from_config(&config)
        .await
        .with_context(error_ctx!("Failed to create application state"))?;

    let router = routes::make_router().with_state(state);

    Ok(router)
}
