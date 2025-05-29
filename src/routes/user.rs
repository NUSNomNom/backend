use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub(crate) async fn create<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

pub(crate) async fn fetch<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

pub(crate) async fn update_password<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

pub(crate) async fn reset_password_request<S: AppState>(State(_): State<S>) -> impl IntoResponse {}
