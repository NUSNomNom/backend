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
    let db_review = fetch_review_by_id(db, review_id).await?;
    Ok(db_review.into())
}

async fn fetch_review_by_id(
    db: &MySqlPool,
    review_id: i64,
) -> Result<DbReview, (StatusCode, &'static str)> {
    sqlx::query_as!(
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_fetch_review_by_id_success(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Test review')"#
        )
        .execute(&db)
        .await
        .unwrap();

        // Find the review by its unique content
        let review_id = i64::from(
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'Test review' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id,
        );

        let result = fetch_review_by_id(&db, review_id).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.review_id, review_id);
        assert_eq!(review.store_id, 1);
        assert_eq!(review.nomer_id, 1);
        assert_eq!(review.score, 5);
        assert_eq!(review.comment, "Test review");
    }

    #[sqlx::test]
    async fn test_fetch_review_by_id_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = fetch_review_by_id(&db, 999).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_read_one_review_success(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (2, 2, 4, 'Another test review')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id = i64::from(
            sqlx::query!(
                "SELECT review_id FROM review WHERE comment = 'Another test review' LIMIT 1"
            )
            .fetch_one(&db)
            .await
            .unwrap()
            .review_id,
        );

        let result = read_one_review(&db, review_id).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.id, review_id);
        assert_eq!(review.store_id, 2);
        assert_eq!(review.nomer_id, 2);
        assert_eq!(review.score, 4);
        assert_eq!(review.comment, "Another test review");
    }

    #[sqlx::test]
    async fn test_read_one_review_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = read_one_review(&db, 999).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_multiple_reviews_fetch_correct_one(db: MySqlPool) {
        setup_test_data(&db).await;

        // Insert multiple reviews and collect their IDs
        let mut review_data = Vec::new();

        for i in 1..=3 {
            let comment = format!("Review {i}");
            sqlx::query!(
                r#"INSERT INTO review (store_id, nomer_id, score, comment) 
                   VALUES (?, ?, ?, ?)"#,
                i,
                i,
                i + 2,
                comment
            )
            .execute(&db)
            .await
            .unwrap();

            let review_id = i64::from(
                sqlx::query!(
                    "SELECT review_id FROM review WHERE comment = ? LIMIT 1",
                    comment
                )
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id,
            );

            review_data.push((review_id, i, comment));
        }

        // Test fetching each review individually
        for (review_id, i, comment) in review_data {
            let review = read_one_review(&db, review_id).await.unwrap();
            assert_eq!(review.id, review_id);
            assert_eq!(review.store_id, i);
            assert_eq!(review.nomer_id, i);
            assert_eq!(review.score, i + 2);
            assert_eq!(review.comment, comment);
        }
    }

    #[sqlx::test]
    async fn test_review_with_special_characters(db: MySqlPool) {
        setup_test_data(&db).await;

        let special_comment = "Review with special chars: !@#$%^&*()_+{}|:<>?[]\\;',./";

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 3, ?)"#,
            special_comment
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id = i64::from(
            sqlx::query!(
                "SELECT review_id FROM review WHERE comment = ? LIMIT 1",
                special_comment
            )
            .fetch_one(&db)
            .await
            .unwrap()
            .review_id,
        );

        let review = read_one_review(&db, review_id).await.unwrap();
        assert_eq!(review.comment, special_comment);
    }

    #[sqlx::test]
    async fn test_review_with_unicode_characters(db: MySqlPool) {
        setup_test_data(&db).await;

        let unicode_comment = "Unicode review: 你好世界 🍜 مرحبا بالعالم 🌍";

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, ?)"#,
            unicode_comment
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id = i64::from(
            sqlx::query!(
                "SELECT review_id FROM review WHERE comment = ? LIMIT 1",
                unicode_comment
            )
            .fetch_one(&db)
            .await
            .unwrap()
            .review_id,
        );

        let review = read_one_review(&db, review_id).await.unwrap();
        assert_eq!(review.comment, unicode_comment);
    }

    #[sqlx::test]
    async fn test_review_boundary_scores(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test minimum score
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 1, 'Minimum score')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let min_review_id = i64::from(
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'Minimum score' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id,
        );

        let min_review = read_one_review(&db, min_review_id).await.unwrap();
        assert_eq!(min_review.score, 1);

        // Test maximum score
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Maximum score')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let max_review_id = i64::from(
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'Maximum score' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id,
        );

        let max_review = read_one_review(&db, max_review_id).await.unwrap();
        assert_eq!(max_review.score, 5);
    }

    #[sqlx::test]
    async fn test_db_review_to_review_conversion(db: MySqlPool) {
        setup_test_data(&db).await;

        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 4, 'Conversion test')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id = i64::from(
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'Conversion test' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id,
        );

        let db_review = fetch_review_by_id(&db, review_id).await.unwrap();
        let review: Review = db_review.into();

        assert_eq!(review.id, review_id);
        assert_eq!(review.store_id, 1);
        assert_eq!(review.nomer_id, 1);
        assert_eq!(review.score, 4);
        assert_eq!(review.comment, "Conversion test");
        // created_at should be set automatically
        assert!(review.created_at <= chrono::Utc::now());
    }

    #[sqlx::test]
    async fn test_error_handling_database_errors(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test with negative ID (invalid but won't cause DB error)
        let result = fetch_review_by_id(&db, -1).await;
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");

        // Test with zero ID
        let result = fetch_review_by_id(&db, 0).await;
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
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
