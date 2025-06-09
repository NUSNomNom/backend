use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::{models::Location, state::AppState};

pub(super) async fn handle(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_as!(
        Location,
        r#"SELECT
            Id as id,
            Name as name,
            Longitude as longitude,
            Latitude as latitude
        FROM Location"#
    )
    .fetch_all(state.db())
    .await
    {
        Ok(locations) => (StatusCode::OK, Json(locations)).into_response(),
        Err(err) => {
            error!("Failed to fetch locations: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
