mod remove;
mod read_one;
mod read_many;
mod create;
use axum::{routing::{delete, get, post}, Router};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::handle))
        .route("/", get(read_many::handle))
        .route("/:id", get(read_one::handle))
        .route("/:id", delete(remove::handle))
}