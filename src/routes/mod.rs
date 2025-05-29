mod user;

use axum::Router;

use crate::state::AppState;

pub(super) fn make_router<S: AppState>() -> Router<S> {
    Router::new().nest("/user", user::make_router::<S>())
}
