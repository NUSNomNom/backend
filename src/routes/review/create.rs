use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::MySqlPool;

use super::DbReview;
use crate::{
    models::{Nomer, Review},
    state::AppState,
};

pub(super) async fn handle(
    State(state): State<AppState>,
    nomer: Nomer,
    body: Json<CreateReviewRequest>,
) -> impl IntoResponse {
    match create_review(
        state.db(),
        body.store_id,
        nomer.id,
        body.score,
        body.comment.clone(),
    )
    .await
    {
        Ok(review) => (StatusCode::CREATED, Json(review)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateReviewRequest {
    store_id: i64,
    score: i64,
    comment: String,
}

async fn create_review(
    db: &MySqlPool,
    store_id: i64,
    nomer_id: i64,
    score: i64,
    comment: String,
) -> Result<Review, (StatusCode, &'static str)> {
    let created_at = Utc::now().naive_utc();

    sqlx::query!(
        r#"
        INSERT INTO review (store_id, nomer_id, score, comment, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
        store_id,
        nomer_id,
        score,
        comment,
        created_at
    )
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create review"))?;

    let review = sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, score, comment, nomer_id, store_id, created_at
        FROM review
        WHERE review_id = LAST_INSERT_ID()
        "#,
    )
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch review"))?;

    Ok(Review {
        id: review.review_id,
        nomer_id: review.nomer_id,
        store_id: review.store_id,
        score: review.score,
        comment: review.comment,
        created_at: review.created_at,
    })
}
