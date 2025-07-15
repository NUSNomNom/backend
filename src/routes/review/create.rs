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
    validate_review_input(store_id, nomer_id, score, &comment)?;
    
    let review_id = insert_review(db, store_id, nomer_id, score, comment).await?;
    let db_review = fetch_created_review(db, review_id).await?;
    
    Ok(db_review.into())
}

fn validate_review_input(
    store_id: i64,
    nomer_id: i64,
    score: i64,
    comment: &str,
) -> Result<(), (StatusCode, &'static str)> {
    if store_id <= 0 {
        return Err((StatusCode::BAD_REQUEST, "Invalid store ID"));
    }
    
    if nomer_id <= 0 {
        return Err((StatusCode::BAD_REQUEST, "Invalid user ID"));
    }
    
    if !(1..=5).contains(&score) {
        return Err((StatusCode::BAD_REQUEST, "Score must be between 1 and 5"));
    }
    
    if comment.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Comment cannot be empty"));
    }
    
    if comment.len() > 255 {
        return Err((StatusCode::BAD_REQUEST, "Comment too long"));
    }
    
    Ok(())
}

async fn insert_review(
    db: &MySqlPool,
    store_id: i64,
    nomer_id: i64,
    score: i64,
    comment: String,
) -> Result<i64, (StatusCode, &'static str)> {
    let created_at = Utc::now().naive_utc();

    let result = sqlx::query!(
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
    .execute(db)
    .await
    .map_err(|e| {
        match e {
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                (StatusCode::BAD_REQUEST, "Invalid store or user ID")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create review")
        }
    })?;

    // Get the ID of the inserted review from the execute result
    let review_id = result.last_insert_id() as i64;

    if review_id == 0 {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get review ID"));
    }

    Ok(review_id)
}

async fn fetch_created_review(
    db: &MySqlPool,
    review_id: i64,
) -> Result<DbReview, (StatusCode, &'static str)> {
    sqlx::query_as!(
        DbReview,
        r#"
        SELECT review_id, score, comment, nomer_id, store_id, created_at
        FROM review
        WHERE review_id = ?
        "#,
        review_id
    )
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch created review"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_review_input_valid() {
        let result = validate_review_input(1, 1, 3, "Good food");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_review_input_invalid_store_id() {
        let result = validate_review_input(0, 1, 3, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Invalid store ID");

        let result = validate_review_input(-1, 1, 3, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Invalid store ID");
    }

    #[test]
    fn test_validate_review_input_invalid_nomer_id() {
        let result = validate_review_input(1, 0, 3, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Invalid user ID");

        let result = validate_review_input(1, -1, 3, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Invalid user ID");
    }

    #[test]
    fn test_validate_review_input_invalid_score() {
        let result = validate_review_input(1, 1, 0, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Score must be between 1 and 5");

        let result = validate_review_input(1, 1, 6, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Score must be between 1 and 5");

        let result = validate_review_input(1, 1, -1, "Good food");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Score must be between 1 and 5");
    }

    #[test]
    fn test_validate_review_input_empty_comment() {
        let result = validate_review_input(1, 1, 3, "");
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Comment cannot be empty");
    }

    #[test]
    fn test_validate_review_input_comment_too_long() {
        let long_comment = "a".repeat(256);
        let result = validate_review_input(1, 1, 3, &long_comment);
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Comment too long");
    }

    #[test]
    fn test_validate_review_input_boundary_values() {
        // Test valid boundary values
        assert!(validate_review_input(1, 1, 1, "x").is_ok());
        assert!(validate_review_input(1, 1, 5, "x").is_ok());
        
        let max_length_comment = "a".repeat(255);
        assert!(validate_review_input(1, 1, 3, &max_length_comment).is_ok());
    }

    #[sqlx::test]
    async fn test_insert_review_success(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = insert_review(&db, 1, 1, 4, "Great food!".to_string()).await;
        assert!(result.is_ok());

        let review_id = result.unwrap();
        assert!(review_id > 0);

        // Verify the review was actually inserted
        let review = sqlx::query!(
            "SELECT * FROM review WHERE review_id = ?",
            review_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(review.store_id, 1);
        assert_eq!(review.nomer_id, 1);
        assert_eq!(review.score, 4);
        assert_eq!(review.comment, "Great food!");
    }

    #[sqlx::test]
    async fn test_insert_review_foreign_key_violation(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test with non-existent store_id
        let result = insert_review(&db, 999, 1, 4, "Test".to_string()).await;
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Invalid store or user ID");

        // Test with non-existent nomer_id
        let result = insert_review(&db, 1, 999, 4, "Test".to_string()).await;
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(message, "Invalid store or user ID");
    }

    #[sqlx::test]
    async fn test_fetch_created_review_success(db: MySqlPool) {
        setup_test_data(&db).await;

        // First insert a review
        let review_id = insert_review(&db, 1, 1, 5, "Excellent!".to_string()).await.unwrap();

        // Then fetch it
        let result = fetch_created_review(&db, review_id).await;
        assert!(result.is_ok());

        let db_review = result.unwrap();
        assert_eq!(db_review.review_id, review_id);
        assert_eq!(db_review.store_id, 1);
        assert_eq!(db_review.nomer_id, 1);
        assert_eq!(db_review.score, 5);
        assert_eq!(db_review.comment, "Excellent!");
    }

    #[sqlx::test]
    async fn test_fetch_created_review_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = fetch_created_review(&db, 999).await;
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(message, "Failed to fetch created review");
    }

    #[sqlx::test]
    async fn test_create_review_success(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = create_review(&db, 1, 1, 4, "Delicious food!".to_string()).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.store_id, 1);
        assert_eq!(review.nomer_id, 1);
        assert_eq!(review.score, 4);
        assert_eq!(review.comment, "Delicious food!");
        assert!(review.id > 0);
        // created_at should be within the last minute (generous window for tests)
        let now = chrono::Utc::now();
        let one_minute_ago = now - chrono::Duration::minutes(1);
        assert!(review.created_at >= one_minute_ago);
        assert!(review.created_at <= now + chrono::Duration::seconds(5));
    }

    #[sqlx::test]
    async fn test_create_review_validation_errors(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test invalid store_id
        let result = create_review(&db, 0, 1, 4, "Test".to_string()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);

        // Test invalid score
        let result = create_review(&db, 1, 1, 0, "Test".to_string()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);

        // Test empty comment
        let result = create_review(&db, 1, 1, 4, "".to_string()).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[sqlx::test]
    async fn test_create_review_with_special_characters(db: MySqlPool) {
        setup_test_data(&db).await;

        let special_comment = "Great food! Special chars: !@#$%^&*()_+{}|:<>?[]\\;',./";
        let result = create_review(&db, 1, 1, 5, special_comment.to_string()).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.comment, special_comment);
    }

    #[sqlx::test]
    async fn test_create_review_with_unicode_characters(db: MySqlPool) {
        setup_test_data(&db).await;

        let unicode_comment = "美食! Delicious! 맛있어요! 🍜 🌟";
        let result = create_review(&db, 1, 1, 5, unicode_comment.to_string()).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.comment, unicode_comment);
    }

    #[sqlx::test]
    async fn test_create_review_boundary_scores(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test minimum score
        let result = create_review(&db, 1, 1, 1, "Poor".to_string()).await;
        assert!(result.is_ok());
        let review = result.unwrap();
        assert_eq!(review.score, 1);

        // Test maximum score
        let result = create_review(&db, 1, 2, 5, "Excellent".to_string()).await;
        assert!(result.is_ok());
        let review = result.unwrap();
        assert_eq!(review.score, 5);
    }

    #[sqlx::test]
    async fn test_create_review_max_length_comment(db: MySqlPool) {
        setup_test_data(&db).await;

        let max_comment = "x".repeat(255);
        let result = create_review(&db, 1, 1, 3, max_comment.clone()).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.comment, max_comment);
        assert_eq!(review.comment.len(), 255);
    }

    #[sqlx::test]
    async fn test_create_multiple_reviews_same_user_store(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create first review
        let result1 = create_review(&db, 1, 1, 3, "First review".to_string()).await;
        assert!(result1.is_ok());
        let review1 = result1.unwrap();

        // Create second review from same user for same store
        let result2 = create_review(&db, 1, 1, 5, "Updated review".to_string()).await;
        assert!(result2.is_ok());
        let review2 = result2.unwrap();

        // Both reviews should exist and have different IDs
        assert_ne!(review1.id, review2.id);
        assert_eq!(review1.store_id, review2.store_id);
        assert_eq!(review1.nomer_id, review2.nomer_id);
    }

    #[sqlx::test]
    async fn test_create_review_time_accuracy(db: MySqlPool) {
        setup_test_data(&db).await;

        let before_creation = chrono::Utc::now();
        let result = create_review(&db, 1, 1, 4, "Time test".to_string()).await;
        let after_creation = chrono::Utc::now() + chrono::Duration::seconds(5); // Add buffer for DB operations

        assert!(result.is_ok());
        let review = result.unwrap();
        
        // The created_at timestamp should be within a reasonable range
        // Database CURRENT_TIMESTAMP might be slightly different from our timestamps
        let one_minute_ago = before_creation - chrono::Duration::minutes(1);
        assert!(review.created_at >= one_minute_ago);
        assert!(review.created_at <= after_creation);
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
