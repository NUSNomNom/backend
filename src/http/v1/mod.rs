mod user;

use axum::Router;

use crate::http::AppState;

pub fn make_router() -> Router<AppState> {
    Router::new().nest("user", user::make_router())
}
