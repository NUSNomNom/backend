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
        .route("/{id}", get(read_one::handle))
        .route("/{id}", delete(remove::handle))
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

impl From<DbReview> for crate::models::Review {
    fn from(db_review: DbReview) -> Self {
        Self {
            id: db_review.review_id,
            store_id: db_review.store_id,
            nomer_id: db_review.nomer_id,
            score: db_review.score,
            comment: db_review.comment,
            created_at: db_review.created_at,
        }
    }
}

impl From<crate::models::Review> for DbReview {
    fn from(review: crate::models::Review) -> Self {
        Self {
            review_id: review.id,
            store_id: review.store_id,
            nomer_id: review.nomer_id,
            score: review.score,
            comment: review.comment,
            created_at: review.created_at,
        }
    }
}
