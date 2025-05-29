mod create;
mod fetch;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub(super) fn make_router<S: AppState>() -> Router<S> {
    Router::new()
        .route("/", post(create::handle::<S>))
        .route("/", get(fetch::handle::<S>))
}
