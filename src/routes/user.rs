use axum::{
    Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post, put},
};

use crate::state::AppState;

pub(super) fn make_router<S: AppState>() -> axum::Router<S> {
    Router::new()
        .route("/", post(create::<S>))
        .route("/", get(fetch::<S>))
        .route("/password", put(update_password::<S>))
        .route("/recovery", post(request_reset::<S>))
}

async fn create<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

async fn fetch<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

async fn update_password<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

async fn request_reset<S: AppState>(State(_): State<S>) -> impl IntoResponse {}