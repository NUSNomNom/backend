mod create;
mod fetch;
mod fetch_public;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::handle))
        .route("/", get(fetch::handle))
        .route("/{id}", get(fetch_public::handle))
}
