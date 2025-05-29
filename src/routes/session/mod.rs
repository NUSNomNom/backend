use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

mod login;
mod refresh;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(login::handle))
        .route("/", get(refresh::handle))
}
