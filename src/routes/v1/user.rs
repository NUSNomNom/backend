use axum::{
    extract::State,
    response::IntoResponse,
};

use crate::AppState;

pub async fn create<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

pub async fn fetch<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

pub async fn update_password<S: AppState>(State(_): State<S>) -> impl IntoResponse {}

pub async fn reset_password_request<S: AppState>(State(_): State<S>) -> impl IntoResponse {}