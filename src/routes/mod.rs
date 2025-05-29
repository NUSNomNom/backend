mod user;

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::state::AppState;

pub(crate) fn make_router<S: AppState>() -> Router<S> {
    Router::new()
        .route("/api/user", post(user::create::<S>))
        .route("/api/user", get(user::fetch::<S>))
        .route("/api/user/password", put(user::update_password::<S>))
        .route(
            "/api/user/recovery",
            post(user::reset_password_request::<S>),
        )
}
