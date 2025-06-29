use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bigdecimal::BigDecimal;
use sqlx::MySqlPool;

use crate::{
    models::{Canteen, Item, Store},
    state::AppState,
};

pub(super) async fn handle(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_data(state.db()).await {
        Ok(locations) => (StatusCode::OK, Json(locations)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

struct DbCanteen {
    canteen_id: i64,
    canteen_name: String,
    latitude: BigDecimal,
    longitude: BigDecimal,
    image_url: String,
}

struct DbStore {
    store_id: i64,
    canteen_id: i64,
    store_name: String,
    is_open: bool,
    cuisine: String,
    information: String,
    image_url: String,
}

struct DbItem {
    item_id: i64,
    store_id: i64,
    item_name: String,
    price: BigDecimal,
    is_available: bool,
    information: String,
    image_url: String,
}

async fn get_all_data(db: &MySqlPool) -> Result<Vec<Canteen>, (StatusCode, &'static str)> {
    let locations = fetch_all_locations(db).await?;
    let stores = fetch_all_stores(db).await?;
    let items = fetch_all_items(db).await?;

    let locations = locations
        .into_iter()
        .map(|cant| {
            let stores_at_loc = stores
                .iter()
                .filter(|&st| st.canteen_id == cant.canteen_id)
                .map(|sto| {
                    let items_at_store = items
                        .iter()
                        .filter(|&ite| ite.store_id == sto.store_id)
                        .map(|ite| Item {
                            id: ite.item_id,
                            name: ite.item_name.clone(),
                            price: ite.price.clone(),
                            is_available: ite.is_available,
                            information: ite.information.clone(),
                            image_url: ite.image_url.clone(),
                        })
                        .collect::<Vec<Item>>();

                    Store {
                        id: sto.store_id,
                        name: sto.store_name.clone(),
                        is_open: sto.is_open,
                        cuisine: sto.cuisine.clone(),
                        information: sto.information.clone(),
                        image_url: sto.image_url.clone(),
                        items: items_at_store,
                    }
                })
                .collect::<Vec<Store>>();

            Canteen {
                id: cant.canteen_id,
                name: cant.canteen_name.clone(),
                latitude: cant.latitude.clone(),
                longitude: cant.longitude.clone(),
                image_url: cant.image_url.clone(),
                stores: stores_at_loc,
            }
        })
        .collect::<Vec<Canteen>>();

    Ok(locations)
}

async fn fetch_all_locations(db: &MySqlPool) -> Result<Vec<DbCanteen>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbCanteen,
        r#"
        SELECT
            canteen_id,
            canteen_name,
            latitude,
            longitude,
            image_url
        FROM canteen
        ORDER BY canteen_id
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
            store_id,
            store_name,
            is_open as `is_open: bool`,
            cuisine,
            information,
            canteen_id,
            image_url
        FROM store
        ORDER BY store_id
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
            item_id,
            item_name,
            price,
            is_available as `is_available: bool`,
            information,
            store_id,
            image_url
        FROM item
        ORDER BY item_id
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
}
