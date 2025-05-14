//! User endpoints
//!
//! Used for registration, password reset, etc.
//!
//! ### Endpoints
//!
//! 1. `POST /api/v1/user`
//!
//! Create a new user. Details to be added.
//!
//! 2. `GET /api/v1/user`
//!
//! Get basic information of an authenticated user. Details to be added.
//!
//! 3. `PUT /api/v1/user/password`
//!
//! Change password of an existing user. Details to be added.
//!
//! 4. `POST /api/v1/user/recovery`
//!
//! Obtain password reset token of an existing user. Details to be added.
use axum::{
    Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post, put},
};

use crate::http::AppState;

pub fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(post_user))
        .route("/", get(get_user))
        .route("password", put(update_password))
        .route("recovery", post(reset_password_request))
}

async fn post_user(State(_): State<AppState>) -> impl IntoResponse {}

async fn get_user(State(_): State<AppState>) -> impl IntoResponse {}

async fn update_password(State(_): State<AppState>) -> impl IntoResponse {}

async fn reset_password_request(State(_): State<AppState>) -> impl IntoResponse {}
