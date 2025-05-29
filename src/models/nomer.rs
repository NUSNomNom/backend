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
        Claim::make(self.email.clone(), 60 * 60)
            .sign_with_key(key)
            .ok()
    }

    pub fn make_refresh_token(&self, key: &Hmac<Sha256>) -> Option<String> {
        Claim::make(self.email.clone(), 60 * 60 * 24 * 30)
            .sign_with_key(key)
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
    sub: String,
    exp: i64,
    iat: i64,
}

impl Claim {
    fn make(subject: String, duration: i64) -> Self {
        let now = Utc::now().timestamp();
        Claim {
            sub: subject,
            exp: now + duration,
            iat: now,
        }
    }
}
