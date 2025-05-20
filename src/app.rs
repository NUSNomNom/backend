use anyhow::{Context, Result};
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};

use crate::{
    config::Config,
    error_ctx, routes,
    state::{AppState, DefaultState},
};

async fn make_app(config: &Config) -> Result<Router<()>> {
    // Initialise application state
    let state = DefaultState::from_config(config)
        .await
        .with_context(error_ctx!("Failed to create application state"))?;

    // Double nesting here to allow for v2
    let router = routes::make_router()
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .with_state(state);

    Ok(router)
}

#[tracing::instrument]
pub(crate) async fn serve(config: Config, listener: TcpListener) -> Result<()> {
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
