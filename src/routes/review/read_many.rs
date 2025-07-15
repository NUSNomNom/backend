use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::MySqlPool;

use super::DbReview;
use crate::{models::Review, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ReviewFilters {
    pub nomer_id: Option<i64>,
    pub store_id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub(super) async fn handle(
    State(state): State<AppState>,
    Query(filters): Query<ReviewFilters>,
) -> impl IntoResponse {
    match read_many_reviews(state.db(), filters).await {
        Ok(reviews) => (StatusCode::OK, Json(reviews)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

async fn read_many_reviews(
    db: &MySqlPool,
    filters: ReviewFilters,
) -> Result<Vec<Review>, (StatusCode, &'static str)> {
    let limit = validate_limit(filters.limit);
    let offset = filters.offset.unwrap_or(0);

    let db_reviews = match (filters.nomer_id, filters.store_id) {
        (Some(nomer_id), Some(store_id)) => {
            fetch_reviews_with_both_filters(db, nomer_id, store_id, limit, offset).await
        }
        (Some(nomer_id), None) => fetch_reviews_by_nomer(db, nomer_id, limit, offset).await,
        (None, Some(store_id)) => fetch_reviews_by_store(db, store_id, limit, offset).await,
        (None, None) => fetch_all_reviews(db, limit, offset).await,
    }?;

    Ok(db_reviews
        .into_iter()
        .map(|db_review| db_review.into())
        .collect())
}

fn validate_limit(limit: Option<i64>) -> i64 {
    limit.unwrap_or(50).min(100).max(1)
}

async fn fetch_reviews_with_both_filters(
    db: &MySqlPool,
    nomer_id: i64,
    store_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<DbReview>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, store_id, nomer_id, score, comment, created_at
        FROM review
        WHERE nomer_id = ? AND store_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        nomer_id,
        store_id,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch reviews"))
}

async fn fetch_reviews_by_nomer(
    db: &MySqlPool,
    nomer_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<DbReview>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, store_id, nomer_id, score, comment, created_at
        FROM review
        WHERE nomer_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        nomer_id,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch reviews"))
}

async fn fetch_reviews_by_store(
    db: &MySqlPool,
    store_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<DbReview>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, store_id, nomer_id, score, comment, created_at
        FROM review
        WHERE store_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        store_id,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch reviews"))
}

async fn fetch_all_reviews(
    db: &MySqlPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<DbReview>, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, store_id, nomer_id, score, comment, created_at
        FROM review
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch reviews"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_limit() {
        assert_eq!(validate_limit(None), 50);

        assert_eq!(validate_limit(Some(25)), 25);
        assert_eq!(validate_limit(Some(100)), 100);

        assert_eq!(validate_limit(Some(150)), 100);

        assert_eq!(validate_limit(Some(0)), 1);
        assert_eq!(validate_limit(Some(-5)), 1);
    }

    #[sqlx::test]
    async fn test_fetch_all_reviews(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Excellent'), (2, 2, 4, 'Good'), (3, 3, 3, 'Average')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let reviews = fetch_all_reviews(&db, 10, 0).await.unwrap();

        assert_eq!(reviews.len(), 3);
        let comments: Vec<&String> = reviews.iter().map(|r| &r.comment).collect();
        assert!(comments.contains(&&"Excellent".to_string()));
        assert!(comments.contains(&&"Good".to_string()));
        assert!(comments.contains(&&"Average".to_string()));
    }

    #[sqlx::test]
    async fn test_fetch_reviews_by_nomer(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'User 1 Review 1'), (2, 1, 4, 'User 1 Review 2'), (3, 2, 3, 'User 2 Review')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let reviews = fetch_reviews_by_nomer(&db, 1, 10, 0).await.unwrap();

        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].nomer_id, 1);
        assert_eq!(reviews[1].nomer_id, 1);
    }

    #[sqlx::test]
    async fn test_fetch_reviews_by_store(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Store 1 Review 1'), (1, 2, 4, 'Store 1 Review 2'), (2, 3, 3, 'Store 2 Review')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let reviews = fetch_reviews_by_store(&db, 1, 10, 0).await.unwrap();

        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].store_id, 1);
        assert_eq!(reviews[1].store_id, 1);
    }

    #[sqlx::test]
    async fn test_fetch_reviews_with_both_filters(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Match'), (1, 2, 4, 'No match - different nomer'), 
                      (2, 1, 3, 'No match - different store'), (2, 2, 2, 'No match - both different')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let reviews = fetch_reviews_with_both_filters(&db, 1, 1, 10, 0)
            .await
            .unwrap();

        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].nomer_id, 1);
        assert_eq!(reviews[0].store_id, 1);
        assert_eq!(reviews[0].comment, "Match");
    }

    #[sqlx::test]
    async fn test_fetch_reviews_pagination(db: MySqlPool) {
        setup_test_data(&db).await;

        for i in 1..=5 {
            sqlx::query!(
                r#"INSERT INTO review (store_id, nomer_id, score, comment) 
                   VALUES (?, 1, 5, ?)"#,
                i.min(3),
                format!("Review {}", i)
            )
            .execute(&db)
            .await
            .unwrap();
        }

        let reviews = fetch_all_reviews(&db, 2, 0).await.unwrap();
        assert_eq!(reviews.len(), 2);

        let reviews = fetch_all_reviews(&db, 2, 2).await.unwrap();
        assert_eq!(reviews.len(), 2);

        let reviews = fetch_all_reviews(&db, 10, 10).await.unwrap();
        assert_eq!(reviews.len(), 0);
    }

    #[sqlx::test]
    async fn test_read_many_reviews_all_filters(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Review 1'), (2, 2, 4, 'Review 2'), (1, 2, 3, 'Review 3')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let filters = ReviewFilters {
            nomer_id: None,
            store_id: None,
            limit: None,
            offset: None,
        };
        let reviews = read_many_reviews(&db, filters).await.unwrap();
        assert_eq!(reviews.len(), 3);

        let filters = ReviewFilters {
            nomer_id: Some(1),
            store_id: None,
            limit: None,
            offset: None,
        };
        let reviews = read_many_reviews(&db, filters).await.unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].nomer_id, 1);

        let filters = ReviewFilters {
            nomer_id: None,
            store_id: Some(1),
            limit: None,
            offset: None,
        };
        let reviews = read_many_reviews(&db, filters).await.unwrap();
        assert_eq!(reviews.len(), 2);
        assert!(reviews.iter().all(|r| r.store_id == 1));

        let filters = ReviewFilters {
            nomer_id: Some(2),
            store_id: Some(1),
            limit: None,
            offset: None,
        };
        let reviews = read_many_reviews(&db, filters).await.unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].nomer_id, 2);
        assert_eq!(reviews[0].store_id, 1);

        let filters = ReviewFilters {
            nomer_id: None,
            store_id: None,
            limit: Some(2),
            offset: None,
        };
        let reviews = read_many_reviews(&db, filters).await.unwrap();
        assert_eq!(reviews.len(), 2);

        let filters = ReviewFilters {
            nomer_id: None,
            store_id: None,
            limit: Some(2),
            offset: Some(1),
        };
        let reviews = read_many_reviews(&db, filters).await.unwrap();
        assert_eq!(reviews.len(), 2);
    }

    async fn setup_test_data(db: &MySqlPool) {
        sqlx::query!(
            r#"INSERT INTO canteen (canteen_name, latitude, longitude, image_url) 
               VALUES ('Test Canteen', 1.0, 103.0, 'test_canteen.jpg')"#
        )
        .execute(db)
        .await
        .unwrap();

        for i in 1..=3 {
            sqlx::query!(
                r#"INSERT INTO store (store_name, is_open, cuisine, information, canteen_id, image_url) 
                   VALUES (?, TRUE, 'Test Cuisine', 'Test Info', 1, ?)"#,
                format!("Test Store {}", i),
                format!("test_store_{}.jpg", i)
            )
            .execute(db)
            .await
            .unwrap();
        }

        for i in 1..=3 {
            sqlx::query!(
                r#"INSERT INTO nomer (display_name, email, password_hash) 
                   VALUES (?, ?, ?)"#,
                format!("Test User {}", i),
                format!("test{}@test.com", i),
                format!("test_hash_{}", i)
            )
            .execute(db)
            .await
            .unwrap();
        }
    }
}
