pub mod auth;
pub mod bots;
pub mod games;
pub mod starterkits;

use std::path::PathBuf;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::session::{self, SessionStore};

#[derive(Clone)]
pub struct AppState {
    pub sessions: SessionStore,
    pub airesources: PathBuf,
}

/// Builds the `/api` router. Static file serving is attached in `main`.
pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/auth/login", post(auth::login))
        .route("/me", get(auth::me))
        .route("/logout", post(auth::logout))
        .route("/starterkits", get(starterkits::list))
        .route("/starterkits/{name}/download", get(starterkits::download))
        .route("/bots/upload", post(bots::upload))
        .route("/users/me/bots", get(bots::my_bots))
        .route("/games", get(games::list))
        .route("/games/{id}", get(games::get_game))
        .fallback(not_found)
        .with_state(state);

    Router::new().nest("/api", api)
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

pub type ApiError = (StatusCode, Json<ErrorBody>);

/// Looks up the email for the session cookie, if there is a valid session.
pub fn session_email(state: &AppState, jar: &CookieJar) -> Option<String> {
    jar.get("halite_session")
        .and_then(|cookie| session::get(&state.sessions, cookie.value()))
}

pub fn error(status: StatusCode, message: impl Into<String>) -> ApiError {
    (
        status,
        Json(ErrorBody {
            error: message.into(),
        }),
    )
}

async fn not_found() -> impl IntoResponse {
    error(StatusCode::NOT_FOUND, "not found")
}
