mod all;
use axum::{Router, routing::get};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new().route("/", get(all::handle))
}
