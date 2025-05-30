use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{models::NomerClaim, state::AppState};

pub(super) async fn handle(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> impl IntoResponse {
    match refresh(state.hmac(), &body.refresh_token) {
        Ok(access_token) => {
            let response = RefreshResponse { access_token };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err((status, message)) => (status, message).into_response(),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RefreshRequest {
    refresh_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RefreshResponse {
    access_token: String,
}

fn refresh(hmac: &Hmac<Sha256>, refresh_token: &str) -> Result<String, (StatusCode, &'static str)> {
    // Validate the refresh token
    let claim: NomerClaim = match refresh_token.verify_with_key(hmac) {
        Ok(claims) => claims,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid refresh token")),
    };

    // Check if the token is a refresh token
    if claim.acc {
        return Err((StatusCode::UNAUTHORIZED, "Invalid refresh token"));
    }

    // Generate a new access token
    let token = NomerClaim::make(claim.sub, 60 * 60, true);

    match token.sign_with_key(hmac) {
        Ok(access_token) => Ok(access_token),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate access token",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hmac::Mac;

    #[test]
    fn test_refresh() {
        let hmac = Hmac::<Sha256>::new_from_slice(b"secret").unwrap();
        let claim = NomerClaim::make("test_user".to_string(), 60, false);
        let refresh_token = claim.sign_with_key(&hmac).unwrap();

        let result = refresh(&hmac, &refresh_token);
        assert!(result.is_ok());

        let access_token = result.unwrap();
        let verified_claim: NomerClaim = access_token.verify_with_key(&hmac).unwrap();
        assert_eq!(verified_claim.sub, "test_user");
        assert!(verified_claim.acc);
    }
}
