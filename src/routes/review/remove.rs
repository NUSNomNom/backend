use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub(super) async fn handle(State(state): State<AppState>) -> impl IntoResponse {}