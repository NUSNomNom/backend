use argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Argon2};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use email_address::EmailAddress;
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing::error;

use crate::state::AppState;

pub(super) async fn handle<S: AppState>(
    State(state): State<S>,
    Json(body): Json<CreateRequest>,
) -> impl IntoResponse {
    match create_user(body, state.db()).await {
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

async fn create_user(body: CreateRequest, db: &SqlitePool) -> Result<String, impl IntoResponse> {
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
        .map(|hash| hash.to_string()).ok()
} 