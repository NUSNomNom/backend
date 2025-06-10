use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::state::AppState;

pub(super) async fn handle(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
}
