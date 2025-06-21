use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bigdecimal::BigDecimal;
use sqlx::MySqlPool;

use crate::{
    models::{Item, Location, Store},
    state::AppState,
};

pub(super) async fn handle(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_data(state.db()).await {
        Ok(locations) => (StatusCode::OK, Json(locations)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

struct DbLocation {
    id: i64,
    name: String,
    latitude: BigDecimal,
    longitude: BigDecimal,
}

struct DbStore {
    id: i64,
    name: String,
    is_open: bool,
    cuisine: String,
    information: String,
    location_id: i64,
}

struct DbItem {
    id: i64,
    name: String,
    price: BigDecimal,
    is_available: bool,
    information: String,
    store_id: i64,
}

async fn get_all_data(db: &MySqlPool) -> Result<Vec<Location>, (StatusCode, &'static str)> {
    let locations = fetch_all_locations(db).await?;
    let stores = fetch_all_stores(db).await?;
    let items = fetch_all_items(db).await?;

    let locations = locations
        .into_iter()
        .map(|loc| {
            let stores_at_loc = stores
                .iter()
                .filter(|&st| st.location_id == loc.id)
                .map(|st| {
                    let items_at_store = items
                        .iter()
                        .filter(|&it| it.store_id == st.id)
                        .map(|st| Item {
                            id: st.id,
                            name: st.name.clone(),
                            price: st.price.clone(),
                            is_available: st.is_available,
                            information: st.information.clone(),
                        })
                        .collect::<Vec<Item>>();

                    Store {
                        id: st.id,
                        name: st.name.clone(),
                        is_open: st.is_open,
                        cuisine: st.cuisine.clone(),
                        information: st.information.clone(),
                        items: items_at_store,
                    }
                })
                .collect::<Vec<Store>>();

            Location {
                id: loc.id,
                name: loc.name.clone(),
                latitude: loc.latitude.clone(),
                longitude: loc.longitude.clone(),
                stores: stores_at_loc,
            }
        })
        .collect::<Vec<Location>>();

    Ok(locations)
}

async fn fetch_all_locations(
    db: &MySqlPool,
) -> Result<Vec<DbLocation>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbLocation,
        r#"
        SELECT
            Id as id,
            Name as name,
            Latitude as latitude,
            Longitude as longitude
        FROM Location
        ORDER BY id
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

async fn fetch_all_stores(db: &MySqlPool) -> Result<Vec<DbStore>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbStore,
        r#"
        SELECT
            Id as id,
            Name as name,
            IsOpen as `is_open: bool`,
            Cuisine as cuisine,
            Information as information,
            LocationId as location_id
        FROM Store
        ORDER BY id
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}

async fn fetch_all_items(db: &MySqlPool) -> Result<Vec<DbItem>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbItem,
        r#"
        SELECT
            Id as id,
            Name as name,
            Price as price,
            IsAvailable as `is_available: bool`,
            Information as information,
            StoreId as store_id
        FROM Item
        ORDER BY id
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}
