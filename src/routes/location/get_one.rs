use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::state::AppState;

pub(super) async fn handle(
    State(state): State<AppState>,
) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Not implemented")
}