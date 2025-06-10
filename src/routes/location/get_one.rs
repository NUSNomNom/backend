use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::{Error, MySqlPool};
use tracing::error;

use crate::{
    models::{Location, Store},
    state::AppState,
};

pub(super) async fn handle(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match get_one(state.db(), id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err((status, message)) => {
            error!("Error fetching location {}: {}", id, message);
            (status, message).into_response()
        }
    }
}

#[derive(Debug, Serialize)]
pub(super) struct GetOneLocationResponse {
    pub id: i64,
    pub name: String,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub stores: Vec<Store>,
}

async fn get_location(db: &MySqlPool, loc_id: i64) -> Result<Location, (StatusCode, &'static str)> {
    sqlx::query_as!(
        Location,
        r#"SELECT
            Id as id,
            Name as name,
            Longitude as longitude,
            Latitude as latitude
        FROM Location
        WHERE Id = ?"#,
        loc_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| if let Error::RowNotFound = e { (StatusCode::NOT_FOUND, "Location not found") } else {
        error!("Failed to fetch location: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })
}

async fn get_stores(db: &MySqlPool, loc_id: i64) -> Result<Vec<Store>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        Store,
        r#"SELECT
            Id as id,
            Name as name,
            IsOpen as `is_open: bool`,
            Cuisine as cuisine,
            Description as description
        FROM Store
        WHERE LocationId = ?"#,
        loc_id
    )
    .fetch_all(db)
    .await
    .map_err(|e| if let Error::RowNotFound = e { (StatusCode::NOT_FOUND, "Stores not found for this location") } else {
        error!("Failed to fetch stores for location {}: {}", loc_id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })
}

async fn get_one(
    db: &MySqlPool,
    loc_id: i64,
) -> Result<GetOneLocationResponse, (StatusCode, &'static str)> {
    let location = get_location(db, loc_id).await?;
    let stores = get_stores(db, loc_id).await?;

    Ok(GetOneLocationResponse {
        id: location.id,
        name: location.name,
        longitude: location.longitude,
        latitude: location.latitude,
        stores,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_get_location(db: MySqlPool) {
        let loc_id = 1;
        let response = get_location(&db, 1).await;
        assert!(response.is_ok());
        let location = response.unwrap();
        assert_eq!(location.id, loc_id);
    }

    #[sqlx::test]
    async fn test_get_stores(db: MySqlPool) {
        let loc_id = 1;
        let response = get_stores(&db, loc_id).await;
        assert!(response.is_ok());
        let stores = response.unwrap();
        assert!(!stores.is_empty());
    }

    #[sqlx::test]
    async fn test_get_one_location(db: MySqlPool) {
        let loc_id = 1;
        let response = get_one(&db, loc_id).await;
        assert!(response.is_ok());
        let location_response = response.unwrap();
        assert_eq!(location_response.id, loc_id);
        assert!(!location_response.stores.is_empty());
    }
}
