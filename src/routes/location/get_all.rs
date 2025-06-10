use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::MySqlPool;
use tracing::error;

use crate::{models::Location, state::AppState};

pub(super) async fn handle(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_locations(state.db()).await {
        Ok(locations) => (StatusCode::OK, Json(locations)).into_response(),
        Err((status, message)) => {
            error!("Error fetching locations: {}", message);
            (status, message).into_response()
        }
    }
}

async fn get_all_locations(
    db: &MySqlPool,
) -> Result<Vec<Location>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        Location,
        r#"SELECT
            Id as id,
            Name as name,
            Longitude as longitude,
            Latitude as latitude
        FROM Location"#
    )
    .fetch_all(db)
    .await
    .map_err(|e| {
        error!("Failed to fetch locations: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_get_all_locations(pool: MySqlPool) {
        let locations = get_all_locations(&pool).await;
        assert!(locations.is_ok());
        let locations = locations.unwrap();
        assert!(!locations.is_empty());
    }
}