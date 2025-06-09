mod get_all;
mod get_one;
use axum::{
    Router,
    routing::{post, put},
};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(get_all::handle))
        .route("/:id", put(get_one::handle))
}
