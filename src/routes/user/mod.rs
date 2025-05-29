mod fetch;
mod create;

use axum::{routing::{get, post}, Router};

use crate::state::AppState;

pub(super) fn make_router<S: AppState>() -> axum::Router<S> {
    Router::new()
        .route("/", post(create::handle::<S>))
        .route("/", get(fetch::handle::<S>))
}