mod location;
mod session;
mod store;
mod user;

use axum::Router;

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::make_router())
        .nest("/session", session::make_router())
        .nest("/location", location::make_router())
        .nest("/store", store::make_router())
}
