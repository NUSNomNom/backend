mod get_one;
mod get_all;
use axum::{routing::{post, put}, Router};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(get_all::handle))
        .route("/:id", put(get_one::handle))
}
