use axum::{
    Router,
    routing::{get, post, put},
};

use crate::state::AppState;

mod v1;

pub(crate) fn make_router<S: AppState>() -> Router<S> {
    Router::new()
        .route("/api/v1/user", post(v1::user::create::<S>))
        .route("/api/v1/user", get(v1::user::fetch::<S>))
        .route("/api/v1/user/password", put(v1::user::update_password::<S>))
        .route(
            "/api/v1/user/recovery",
            post(v1::user::reset_password_request::<S>),
        )
}
