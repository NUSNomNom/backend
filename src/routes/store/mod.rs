mod get_one;
use axum::{Router, routing::get};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new().route("/{id}", get(get_one::handle))
}
