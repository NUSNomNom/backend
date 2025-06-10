use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::{Error, MySqlPool};
use tracing::error;

use crate::{
    models::{Item, Store},
    state::AppState,
};

pub(super) async fn handle(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match get_one_store(state.db(), id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

#[derive(Debug, Serialize)]
struct GetOneStoreResponse {
    pub id: i64,
    pub name: String,
    pub is_open: bool,
    pub cuisine: String,
    pub items: Vec<Item>,
}

async fn get_store(db: &MySqlPool, store_id: i64) -> Result<Store, (StatusCode, &'static str)> {
    sqlx::query_as!(
        Store,
        r#"SELECT
            Id as id,
            Name as name,
            IsOpen as `is_open: bool`,
            Cuisine as cuisine,
            Description as description
        FROM Store
        WHERE Id = ?"#,
        store_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| if let Error::RowNotFound = e { (StatusCode::NOT_FOUND, "Store not found") } else {
        error!("Failed to fetch store: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })
}

async fn get_items(db: &MySqlPool, store_id: i64) -> Result<Vec<Item>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        Item,
        r#"SELECT
            Id as id,
            Name as name,
            Price as price,
            IsAvailable as `is_available: bool`,
            Description as description
        FROM Item
        WHERE StoreId = ?"#,
        store_id
    )
    .fetch_all(db)
    .await
    .map_err(|e| if let Error::RowNotFound = e { (StatusCode::NOT_FOUND, "Items not found") } else {
        error!("Failed to fetch items: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })
}

async fn get_one_store(
    db: &MySqlPool,
    store_id: i64,
) -> Result<GetOneStoreResponse, (StatusCode, &'static str)> {
    let store = get_store(db, store_id).await?;
    let items = get_items(db, store_id).await?;

    Ok(GetOneStoreResponse {
        id: store.id,
        name: store.name,
        is_open: store.is_open,
        cuisine: store.cuisine,
        items,
    })
}
