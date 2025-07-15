use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::MySqlPool;

use crate::{models::Nomer, state::AppState};

pub(super) async fn handle(
    State(state): State<AppState>,
    nomer: Nomer,
    Path(review_id): Path<i64>,
) -> impl IntoResponse {
    match remove_review(state.db(), nomer.id, review_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

async fn remove_review(
    db: &MySqlPool,
    nomer_id: i64,
    review_id: i64,
) -> Result<(), (StatusCode, &'static str)> {
    // First verify the review exists and belongs to the user
    verify_review_ownership(db, nomer_id, review_id).await?;

    // Delete the review
    delete_review_by_id(db, review_id).await?;

    Ok(())
}

async fn verify_review_ownership(
    db: &MySqlPool,
    nomer_id: i64,
    review_id: i64,
) -> Result<(), (StatusCode, &'static str)> {
    let result = sqlx::query!(
        r#"
        SELECT nomer_id
        FROM review
        WHERE review_id = ?
        "#,
        review_id
    )
    .fetch_optional(db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to verify review ownership",
        )
    })?;

    match result {
        Some(row) => {
            if row.nomer_id as i64 == nomer_id {
                Ok(())
            } else {
                Err((
                    StatusCode::FORBIDDEN,
                    "You can only delete your own reviews",
                ))
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Review not found")),
    }
}

async fn delete_review_by_id(
    db: &MySqlPool,
    review_id: i64,
) -> Result<(), (StatusCode, &'static str)> {
    let result = sqlx::query!(
        r#"
        DELETE FROM review
        WHERE review_id = ?
        "#,
        review_id
    )
    .execute(db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete review"))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Review not found"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_verify_review_ownership_success(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create a review
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 4, 'Test ownership')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id =
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'Test ownership' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id as i64;

        let result = verify_review_ownership(&db, 1, review_id).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_verify_review_ownership_forbidden(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create a review by user 1
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 4, 'User 1 review')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id =
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'User 1 review' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id as i64;

        // Try to verify ownership with a different user (user 2)
        let result = verify_review_ownership(&db, 2, review_id).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(message, "You can only delete your own reviews");
    }

    #[sqlx::test]
    async fn test_verify_review_ownership_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = verify_review_ownership(&db, 1, 999).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_delete_review_by_id_success(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create a review
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 3, 'To be deleted')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id =
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'To be deleted' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id as i64;

        let result = delete_review_by_id(&db, review_id).await;
        assert!(result.is_ok());

        // Verify the review was actually deleted
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM review WHERE review_id = ?",
            review_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .count;

        assert_eq!(count, 0);
    }

    #[sqlx::test]
    async fn test_delete_review_by_id_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = delete_review_by_id(&db, 999).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_remove_review_success(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create a review
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 5, 'Remove test')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id =
            sqlx::query!("SELECT review_id FROM review WHERE comment = 'Remove test' LIMIT 1")
                .fetch_one(&db)
                .await
                .unwrap()
                .review_id as i64;

        let result = remove_review(&db, 1, review_id).await;
        assert!(result.is_ok());

        // Verify the review was deleted
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM review WHERE review_id = ?",
            review_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .count;

        assert_eq!(count, 0);
    }

    #[sqlx::test]
    async fn test_remove_review_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = remove_review(&db, 1, 999).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_remove_review_forbidden(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create a review by user 1
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 4, 'User 1 forbidden test')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id = sqlx::query!(
            "SELECT review_id FROM review WHERE comment = 'User 1 forbidden test' LIMIT 1"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .review_id as i64;

        // Try to remove as user 2
        let result = remove_review(&db, 2, review_id).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(message, "You can only delete your own reviews");

        // Verify the review was NOT deleted
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM review WHERE review_id = ?",
            review_id
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .count;

        assert_eq!(count, 1);
    }

    #[sqlx::test]
    async fn test_remove_review_multiple_users_same_store(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create reviews by different users for the same store
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 4, 'User 1 multi test'), (1, 2, 3, 'User 2 multi test')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let user1_review_id = sqlx::query!(
            "SELECT review_id FROM review WHERE comment = 'User 1 multi test' LIMIT 1"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .review_id as i64;

        let user2_review_id = sqlx::query!(
            "SELECT review_id FROM review WHERE comment = 'User 2 multi test' LIMIT 1"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .review_id as i64;

        // User 1 can delete their own review
        let result = remove_review(&db, 1, user1_review_id).await;
        assert!(result.is_ok());

        // User 2 cannot delete user 1's review (even though it's deleted)
        let result = remove_review(&db, 2, user1_review_id).await;
        assert!(result.is_err());
        let (status, _) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);

        // User 2 can delete their own review
        let result = remove_review(&db, 2, user2_review_id).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_remove_review_edge_case_zero_id(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = remove_review(&db, 1, 0).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_remove_review_edge_case_negative_id(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = remove_review(&db, 1, -1).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_remove_review_large_id(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = remove_review(&db, 1, i64::MAX).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "Review not found");
    }

    #[sqlx::test]
    async fn test_remove_review_after_already_deleted(db: MySqlPool) {
        setup_test_data(&db).await;

        // Create and then delete a review
        sqlx::query!(
            r#"INSERT INTO review (store_id, nomer_id, score, comment) 
               VALUES (1, 1, 4, 'Double delete test')"#
        )
        .execute(&db)
        .await
        .unwrap();

        let review_id = sqlx::query!(
            "SELECT review_id FROM review WHERE comment = 'Double delete test' LIMIT 1"
        )
        .fetch_one(&db)
        .await
        .unwrap()
        .review_id as i64;

        // First deletion should succeed
        let result = remove_review(&db, 1, review_id).await;
        assert!(result.is_ok());

        // Second deletion should fail
        let result = remove_review(&db, 1, review_id).await;
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
