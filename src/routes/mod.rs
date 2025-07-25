mod data;
mod review;
mod session;
mod user;

use axum::Router;

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::make_router())
        .nest("/session", session::make_router())
        .nest("/data", data::make_router())
        .nest("/review", review::make_router())
}
