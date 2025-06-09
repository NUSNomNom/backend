use argon2::{Argon2, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::error;

use crate::{models::Nomer, state::AppState};

pub(super) async fn handle(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    match login(&state, &body.email, &body.password).await {
        Ok((access_token, refresh_token)) => {
            let response = LoginResponse {
                access_token,
                refresh_token,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(out) => out.into_response(),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

async fn login(
    state: &AppState,
    email: &str,
    password: &str,
) -> Result<(String, String), (StatusCode, &'static str)> {
    // Fetch the nomer from the database using the provided email
    let nomer = match get_nomer_by_email(state.db(), email).await {
        Ok(Some(nomer)) => nomer,
        Ok(None) => return Err((StatusCode::UNAUTHORIZED, "Invalid email or password")),
        Err(e) => return Err(e),
    };

    // Verify password
    match verify_password(password, &nomer.password_hash) {
        Some(true) => (), // Password is correct
        Some(false) => return Err((StatusCode::UNAUTHORIZED, "Invalid email or password")),
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password verification failed",
            ));
        }
    }

    // Craft response with access and refresh tokens
    let Some(access_token) = nomer.make_access_token(state.hmac()) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create access token",
        ));
    };
    let Some(refresh_token) = nomer.make_refresh_token(state.hmac()) else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create refresh token",
        ));
    };

    Ok((access_token, refresh_token))
}

async fn get_nomer_by_email(
    db: &SqlitePool,
    email: &str,
) -> Result<Option<Nomer>, (StatusCode, &'static str)> {
    match sqlx::query_as!(
        Nomer,
        r#"
        SELECT
            Id as id,
            DisplayName as display_name,
            Email as email,
            PasswordHash as password_hash
        FROM Nomer WHERE Email = ?
        "#,
        email
    )
    .fetch_one(db)
    .await
    {
        Ok(nomer) => Ok(Some(nomer)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => {
            error!("Database error: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

fn verify_password(password: &str, hash: &str) -> Option<bool> {
    let Ok(parsed_hash) = argon2::PasswordHash::new(hash) else {
        // Invalid hash format
        return None;
    };
    Some(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::{
        PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };

    #[test]
    fn test_verify_password() {
        let password = "test_password";
        let salt = SaltString::generate(OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        assert_eq!(verify_password(password, &hash), Some(true));

        let wrong_password = "wrong_password";
        assert_eq!(verify_password(wrong_password, &hash), Some(false));

        let invalid_hash = "invalid_hash_format";
        assert_eq!(verify_password(password, invalid_hash), None);
    }

    #[sqlx::test]
    async fn test_get_nomer_by_email(db: SqlitePool) {
        sqlx::query!(
            r#"INSERT INTO Nomer (
                DisplayName,
                Email,
                PasswordHash
            ) VALUES (
                "Test",
                "test@test.com",
                "test_hash"
            )"#
        )
        .execute(&db)
        .await
        .unwrap();

        let nomer = get_nomer_by_email(&db, "test@test.com").await;

        assert!(nomer.is_ok());

        let nomer = nomer.unwrap();

        assert!(nomer.is_some());
        assert_eq!(nomer.unwrap().display_name, "Test");

        let nomer = get_nomer_by_email(&db, "wrong@email.com").await;

        assert!(nomer.is_ok());

        let nomer = nomer.unwrap();

        assert!(nomer.is_none());
    }
}
