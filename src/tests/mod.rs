use anyhow::{Context, Result};
use axum::Router;
use sqlx::AnyPool;

use crate::{config::Config, error_ctx, routes, state::AppState};

mod user;

#[derive(Clone)]
/// Mock application state for testing purposes.
struct TestState {
    /// Mock database connection pool
    db_pool: AnyPool,
}

impl AppState for TestState {
    async fn from_config(config: &Config) -> Result<Self> {
        let db_pool = AnyPool::connect(&config.database_url)
            .await
            .with_context(error_ctx!("Unable to create an in-memeory SQLite database"))?;
        Ok(Self { db_pool })
    }

    async fn db(&self) -> &AnyPool {
        &self.db_pool
    }
}

async fn make_test_app() -> Result<Router<()>> {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        port: 42069,
    };

    let state = TestState::from_config(&config)
        .await
        .with_context(error_ctx!("Failed to create application state"))?;

    let router = routes::make_router().with_state(state);

    Ok(router)
}