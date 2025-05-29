use axum::{
    routing::{post, put}, Router
};

use crate::state::AppState;

mod login;
mod refresh;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(login::handle))
        .route("/", put(refresh::handle))
}
