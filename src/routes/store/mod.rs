mod get_one;
use axum::{
    Router,
    routing::{post, put},
};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
}