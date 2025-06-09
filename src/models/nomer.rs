use axum::{extract::FromRequestParts, http::{request::Parts, StatusCode}};
use chrono::{NaiveDateTime, Utc};
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Nomer {
    pub id: i64,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Nomer {
    pub fn make_access_token(&self, key: &Hmac<Sha256>) -> Option<String> {
        NomerClaim::make(self.email.clone(), 60 * 60, true)
            .sign_with_key(key)
            .ok()
    }

    pub fn make_refresh_token(&self, key: &Hmac<Sha256>) -> Option<String> {
        NomerClaim::make(self.email.clone(), 60 * 60 * 24 * 30, false)
            .sign_with_key(key)
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NomerClaim {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub acc: bool,
}

impl NomerClaim {
    pub fn make(subject: String, duration: i64, is_access: bool) -> Self {
        let now = Utc::now().timestamp();
        NomerClaim {
            sub: subject,
            exp: now + duration,
            iat: now,
            acc: is_access,
        }
    }
}

impl FromRequestParts<AppState> for NomerClaim {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let Some(api_key_header) = parts.headers.get("X-Api-Key") else {
            // Missing X-Api-Key header
            return Err((StatusCode::UNAUTHORIZED, "Missing X-Api-Key header"));
        };

        let Some(api_key) = api_key_header.to_str().ok() else {
            // Invalid (encoding) X-Api-Key header
            return Err((StatusCode::UNAUTHORIZED, "Invalid X-Api-Key header"));
        };

        let Ok(claim): Result<NomerClaim, _> = api_key.verify_with_key(state.hmac()) else {
            // Valid X-Api-Key header, but invalid API key
            return Err((StatusCode::UNAUTHORIZED, "Invalid API key"));
        };
        
        Ok(claim)
    }
}
