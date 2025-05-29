use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub(super) async fn handle<S: AppState>(State(_): State<S>) -> impl IntoResponse {}
