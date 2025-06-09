use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::Error;
use tracing::error;

use crate::{
    models::{Location, Store},
    state::AppState,
};

pub(super) async fn handle(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // Fetch location by ID from the database
    let location = match sqlx::query_as!(
        Location,
        r#"SELECT
            Id as id,
            Name as name,
            Longitude as longitude,
            Latitude as latitude
        FROM Location
        WHERE Id = ?"#,
        id
    )
    .fetch_one(state.db())
    .await
    {
        Ok(loc) => loc,
        Err(Error::RowNotFound) => {
            return (StatusCode::NOT_FOUND, "Location not found").into_response();
        }
        Err(err) => {
            error!("Failed to fetch location: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Fetch all stores associated with the location
    let stores = match sqlx::query_as!(
        Store,
        r#"SELECT
            Id as id,
            Name as name,
            IsOpen as `is_open: bool`,
            Cuisine as cuisine,
            Description as description
        FROM Store
        WHERE LocationId = ?"#,
        id
    )
    .fetch_all(state.db())
    .await
    {
        Ok(stores) => stores,
        Err(err) => {
            error!("Failed to fetch stores for location {}: {}", id, err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    Json(GetOneLocationResponse {
        id: location.id,
        name: location.name,
        longitude: location.longitude,
        latitude: location.latitude,
        stores,
    })
    .into_response()
}

#[derive(Debug, Serialize)]
pub(super) struct GetOneLocationResponse {
    pub id: i64,
    pub name: String,
    pub longitude: BigDecimal,
    pub latitude: BigDecimal,
    pub stores: Vec<Store>,
}
