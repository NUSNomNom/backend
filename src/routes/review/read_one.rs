use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::MySqlPool;

use super::DbReview;
use crate::{models::Review, state::AppState};

pub(super) async fn handle(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match read_one_review(state.db(), id).await {
        Ok(review) => (StatusCode::OK, Json(review)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

async fn read_one_review(
    db: &MySqlPool,
    review_id: i64,
) -> Result<Review, (StatusCode, &'static str)> {
    let review = sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, store_id, nomer_id, score, comment, created_at
        FROM review
        WHERE review_id = ?
        "#,
        review_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| {
        if let sqlx::Error::RowNotFound = e {
            (StatusCode::NOT_FOUND, "Review not found")
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch review")
        }
    })?;

    Ok(review.into())
}
