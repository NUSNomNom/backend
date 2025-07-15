mod create;
mod read_many;
mod read_one;
mod remove;
use axum::{
    Router,
    routing::{delete, get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

pub(super) fn make_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::handle))
        .route("/", get(read_many::handle))
        .route("/:id", get(read_one::handle))
        .route("/:id", delete(remove::handle))
}

#[derive(Debug, Serialize, Deserialize)]
struct DbReview {
    review_id: i64,
    score: i64,
    comment: String,
    nomer_id: i64,
    store_id: i64,
    created_at: DateTime<Utc>,
}
