use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;

use crate::{models::Nomer, state::AppState};

pub(super) async fn handle(State(_): State<AppState>, nomer: Nomer) -> impl IntoResponse {
    Json(FetchResponse {
        id: nomer.id,
        display_name: nomer.display_name,
        email: nomer.email,
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FetchResponse {
    pub id: i64,
    pub display_name: String,
    pub email: String,
}
