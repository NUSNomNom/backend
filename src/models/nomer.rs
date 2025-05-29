use chrono::{NaiveDateTime, Utc};
use hmac::Hmac;
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

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
    sub: String,
    exp: i64,
    iat: i64,
    acc: bool,
}

impl NomerClaim {
    fn make(subject: String, duration: i64, is_access: bool) -> Self {
        let now = Utc::now().timestamp();
        NomerClaim {
            sub: subject,
            exp: now + duration,
            iat: now,
            acc: is_access,
        }
    }

    pub fn is_access_token(&self) -> bool {
        self.acc
    }

    pub fn is_refresh_token(&self) -> bool{
        !self.acc
    }
}
