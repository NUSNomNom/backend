use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};

use crate::{models::NomerClaim, state::AppState};

pub(super) async fn handle(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> impl IntoResponse {
    match refresh(&state, &body.refresh_token).await {
        Ok(access_token) => {
            let response = RefreshResponse { access_token };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err((status, message)) => (status, message).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct RefreshRequest {
    refresh_token: String,
}

#[derive(Debug, Serialize)]
pub(super) struct RefreshResponse {
    access_token: String,
}

async fn refresh(
    state: &AppState,
    refresh_token: &str,
) -> Result<String, (StatusCode, &'static str)> {
    // Validate the refresh token
    let claim: NomerClaim = match refresh_token.verify_with_key(state.hmac()) {
        Ok(claims) => claims,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid refresh token")),
    };

    // Check if the token is a refresh token
    if claim.acc {
        return Err((StatusCode::UNAUTHORIZED, "Invalid refresh token"));
    }

    // Generate a new access token
    let token = NomerClaim::make(claim.sub, 60 * 60, true);

    match token.sign_with_key(state.hmac()) {
        Ok(access_token) => Ok(access_token),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate access token",
        )),
    }
}
