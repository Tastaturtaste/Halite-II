use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

use super::{error, session_email, ApiError, AppState};
use crate::session;

const SESSION_COOKIE: &str = "halite_session";

#[derive(Deserialize)]
pub struct LoginRequest {
    #[serde(default)]
    email: String,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub email: String,
}

/// Any valid-looking email is auto-logged-in; no OTP is sent at this stage.
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, Json<MeResponse>), ApiError> {
    tracing::info!(email = %request.email, "login requested");

    let email = request.email.trim().to_string();
    if email.is_empty() || !email.contains('@') {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "a non-empty email containing '@' is required",
        ));
    }

    let token = session::create(&state.sessions, &email);
    let cookie = Cookie::build((SESSION_COOKIE, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    Ok((jar.add(cookie), Json(MeResponse { email })))
}

pub async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<MeResponse>, ApiError> {
    tracing::info!(
        has_session = jar.get(SESSION_COOKIE).is_some(),
        "session lookup requested"
    );

    match session_email(&state, &jar) {
        Some(email) => Ok(Json(MeResponse { email })),
        None => Err(error(StatusCode::UNAUTHORIZED, "not logged in")),
    }
}

pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> (CookieJar, StatusCode) {
    let removed = jar
        .get(SESSION_COOKIE)
        .map(|cookie| session::remove(&state.sessions, cookie.value()))
        .unwrap_or(false);
    tracing::info!(removed, "logout requested");

    (
        jar.remove(Cookie::from(SESSION_COOKIE)),
        StatusCode::NO_CONTENT,
    )
}
