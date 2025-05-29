use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use email_address::EmailAddress;
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing::error;

use crate::state::AppState;

pub(super) async fn handle(
    State(state): State<AppState>,
    Json(body): Json<CreateRequest>,
) -> impl IntoResponse {
    match create_user(&body, state.db()).await {
        Ok(msg) => (StatusCode::OK, msg).into_response(),
        Err(err) => err.into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateRequest {
    display_name: String,
    email: String,
    password: String,
}

fn validate_display_name(display_name: &str) -> bool {
    display_name.is_empty() || display_name.len() > 50
}

fn validate_email(email: &str) -> bool {
    !EmailAddress::is_valid(email)
}

fn validate_password(password: &str) -> bool {
    password.is_empty() || password.len() < 8 || password.len() > 100
}

fn hash_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();
    hasher
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .ok()
}

async fn create_user(body: &CreateRequest, db: &SqlitePool) -> Result<String, impl IntoResponse> {
    // Validate input
    if validate_display_name(&body.display_name)
        || validate_email(&body.email)
        || validate_password(&body.password)
    {
        return Err((StatusCode::BAD_REQUEST, "Invalid input"));
    }

    // Check display name and email uniqueness
    let display_name_exists = match sqlx::query!(
        "SELECT COUNT(*) AS count FROM Nomer WHERE DisplayName = ? OR Email = ?",
        body.display_name,
        body.email
    )
    .fetch_one(db)
    .await
    {
        Ok(r) => r.count > 0,
        Err(e) => {
            error!("Database query failed: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Database query failed"));
        }
    };
    if display_name_exists {
        return Err((StatusCode::CONFLICT, "Display name or email already exists"));
    }

    // Hash password
    let phc = match hash_password(&body.password) {
        Some(hash) => hash,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password")),
    };

    // Insert user into database
    match sqlx::query!(
        "INSERT INTO Nomer (DisplayName, Email, PasswordHash) VALUES (?, ?, ?)",
        body.display_name,
        body.email,
        phc
    )
    .execute(db)
    .await
    {
        Ok(_) => Ok("User created successfully".to_string()),
        Err(e) => {
            error!("Failed to insert user into database: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user"))
        }
    }
}

#[cfg(test)]
mod tests {
    use argon2::{PasswordHash, PasswordVerifier};

    use super::*;

    #[test]
    fn test_validate_display_name() {
        assert!(validate_display_name(""));
        assert!(validate_display_name(
            "thisisaverylongdisplaynamewhichshouldnotbevalidHAHAHAHHAHAHA"
        ));
        assert!(!validate_display_name("validname"));
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email(""));
        assert!(validate_email("invalid-email"));
        assert!(!validate_email("hello@abc.com"));
        assert!(!validate_email("hello+tag@abc.com"));
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("short"));
        assert!(validate_password(""));
        assert!(!validate_password("validpassword"));
        assert!(!validate_password(
            "thisisaverylongpasswordthatshouldbevalid"
        ));
    }

    #[test]
    fn test_hash_password() {
        let password = "test_password";
        let hashed = hash_password(password);
        let hashed_2 = hash_password(password);

        assert!(hashed.is_some());
        assert!(hashed_2.is_some());
        assert_ne!(hashed, hashed_2);

        let hmac = Argon2::default();
        let hashed_str = hashed.unwrap();
        let hashed = PasswordHash::new(&hashed_str);

        assert!(hashed.is_ok());

        let hashed = hashed.unwrap();
        assert!(hmac.verify_password(password.as_bytes(), &hashed).is_ok());
    }

    #[tokio::test]
    async fn test_create_user() {
        let db = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
        sqlx::migrate!().run(&db).await.unwrap();

        let request = CreateRequest {
            display_name: "test_user".to_string(),
            password: "test_password".to_string(),
            email: "test@test.com".to_string(),
        };
        let result = create_user(&request, &db).await;

        assert!(result.is_ok());

        let existed = create_user(&request, &db).await;

        assert!(existed.is_err());
    }
}
