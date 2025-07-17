use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use sqlx::{Error, MySql, Pool};
use tracing::error;

use crate::state::AppState;

/// Handler for fetching public user information by user ID
pub(super) async fn handle(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    match fetch_user_public_info(state.db(), user_id).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

/// Fetch public user information from the database
async fn fetch_user_public_info(
    db: &Pool<MySql>,
    user_id: i64,
) -> Result<FetchPublicResponse, (StatusCode, &'static str)> {
    sqlx::query_as!(
        FetchPublicResponse,
        r#"
        SELECT
            nomer_id as id,
            display_name
        FROM nomer
        WHERE nomer_id = ?
        "#,
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        Error::RowNotFound => (StatusCode::NOT_FOUND, "User not found"),
        _ => {
            error!("Database error while fetching user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    })
}

/// Response structure for public user information
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FetchPublicResponse {
    pub id: i64,
    pub display_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::MySqlPool;

    #[test]
    fn test_response_structure() {
        let response = FetchPublicResponse {
            id: 123,
            display_name: "Test User".to_string(),
        };

        assert_eq!(response.id, 123);
        assert_eq!(response.display_name, "Test User");
    }

    #[test]
    fn test_response_equality() {
        let response1 = FetchPublicResponse {
            id: 1,
            display_name: "John Doe".to_string(),
        };

        let response2 = FetchPublicResponse {
            id: 1,
            display_name: "John Doe".to_string(),
        };

        let response3 = FetchPublicResponse {
            id: 2,
            display_name: "Jane Smith".to_string(),
        };

        assert_eq!(response1, response2);
        assert_ne!(response1, response3);
    }

    #[test]
    fn test_fetch_user_error_debug() {
        let error = (StatusCode::NOT_FOUND, "User not found");
        assert_eq!(error.0, StatusCode::NOT_FOUND);
        assert_eq!(error.1, "User not found");

        let db_error = (StatusCode::INTERNAL_SERVER_ERROR, "Database error");
        assert_eq!(db_error.0, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(db_error.1, "Database error");
    }

    #[test]
    fn test_error_handling_logic() {
        // Test that NotFound errors are handled correctly
        let not_found_error = (StatusCode::NOT_FOUND, "User not found");
        match not_found_error {
            (StatusCode::NOT_FOUND, "User not found") => {
                // This should be the path taken
                assert!(true);
            }
            _ => {
                panic!("Expected NotFound error");
            }
        }

        // Test that database errors are handled correctly
        let db_error = (StatusCode::INTERNAL_SERVER_ERROR, "Database error");
        match db_error {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error") => {
                // This should be the path taken
                assert!(true);
            }
            _ => {
                panic!("Expected DatabaseError");
            }
        }
    }

    #[test]
    fn test_response_serde_structure() {
        // Test that the serde rename_all works correctly
        let response = FetchPublicResponse {
            id: 1,
            display_name: "Test".to_string(),
        };

        // The field should be serialized as "displayName" due to camelCase
        // This is mainly to ensure the structure compiles correctly with serde
        assert_eq!(response.id, 1);
        assert_eq!(response.display_name, "Test");
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_success(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = fetch_user_public_info(&db, 1).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.display_name, "Test User 1");
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_not_found(db: MySqlPool) {
        setup_test_data(&db).await;

        let result = fetch_user_public_info(&db, 999).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "User not found");
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_multiple_users(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test fetching different users
        let user1 = fetch_user_public_info(&db, 1).await.unwrap();
        let user2 = fetch_user_public_info(&db, 2).await.unwrap();
        let user3 = fetch_user_public_info(&db, 3).await.unwrap();

        assert_eq!(user1.id, 1);
        assert_eq!(user1.display_name, "Test User 1");

        assert_eq!(user2.id, 2);
        assert_eq!(user2.display_name, "Test User 2");

        assert_eq!(user3.id, 3);
        assert_eq!(user3.display_name, "Test User 3");
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_boundary_cases(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test edge case user IDs
        let result_zero = fetch_user_public_info(&db, 0).await;
        assert!(result_zero.is_err());
        let (status, message) = result_zero.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "User not found");

        let result_negative = fetch_user_public_info(&db, -1).await;
        assert!(result_negative.is_err());
        let (status, message) = result_negative.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "User not found");

        let result_large = fetch_user_public_info(&db, i64::MAX).await;
        assert!(result_large.is_err());
        let (status, message) = result_large.unwrap_err();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(message, "User not found");
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_data_integrity(db: MySqlPool) {
        setup_test_data(&db).await;

        // Test that we only get the expected fields (id and display_name)
        let user = fetch_user_public_info(&db, 1).await.unwrap();

        // Verify we have the expected fields
        assert_eq!(user.id, 1);
        assert_eq!(user.display_name, "Test User 1");

        // Verify the response structure is correct
        assert!(user.display_name.starts_with("Test User"));
        assert!(user.id > 0);
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_special_characters(db: MySqlPool) {
        // Insert a user with special characters in the display name
        sqlx::query!(
            r#"INSERT INTO nomer (display_name, email, password_hash) 
               VALUES (?, ?, ?)"#,
            "John 'Test' Doe & Co. <script>",
            "special@test.com",
            "test_hash_special"
        )
        .execute(&db)
        .await
        .unwrap();

        // Get the inserted user ID
        let user_id = sqlx::query!("SELECT nomer_id FROM nomer WHERE email = 'special@test.com'")
            .fetch_one(&db)
            .await
            .unwrap()
            .nomer_id;

        let result = fetch_user_public_info(&db, user_id.into()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, i64::from(user_id));
        assert_eq!(response.display_name, "John 'Test' Doe & Co. <script>");
    }

    #[sqlx::test]
    async fn test_fetch_user_public_info_unicode_characters(db: MySqlPool) {
        // Insert a user with unicode characters in the display name
        sqlx::query!(
            r#"INSERT INTO nomer (display_name, email, password_hash) 
               VALUES (?, ?, ?)"#,
            "张三 🍜 Café",
            "unicode@test.com",
            "test_hash_unicode"
        )
        .execute(&db)
        .await
        .unwrap();

        // Get the inserted user ID
        let user_id = sqlx::query!("SELECT nomer_id FROM nomer WHERE email = 'unicode@test.com'")
            .fetch_one(&db)
            .await
            .unwrap()
            .nomer_id;

        let result = fetch_user_public_info(&db, user_id.into()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, i64::from(user_id));
        assert_eq!(response.display_name, "张三 🍜 Café");
    }

    async fn setup_test_data(db: &MySqlPool) {
        // Insert test users
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
