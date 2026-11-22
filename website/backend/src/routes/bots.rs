use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::Serialize;

use super::{error, session_email, ApiError, AppState};

#[derive(Serialize)]
pub struct UploadResponse {
    pub ok: bool,
}

#[derive(Serialize)]
pub struct Bot {
    pub id: u64,
    pub name: String,
    pub language: String,
    pub rank: u64,
    #[serde(rename = "gamesPlayed")]
    pub games_played: u64,
    #[serde(rename = "uploadDate")]
    pub upload_date: String,
}

#[derive(Serialize)]
pub struct BotsResponse {
    pub bots: Vec<Bot>,
}

pub async fn upload(
    State(state): State<AppState>,
    jar: CookieJar,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, ApiError> {
    let email = session_email(&state, &jar)
        .ok_or_else(|| error(StatusCode::UNAUTHORIZED, "not logged in"))?;

    tracing::info!(email = %email, "bot upload started");

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        error(
            StatusCode::BAD_REQUEST,
            format!("invalid multipart body: {err}"),
        )
    })? {
        let field_name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().unwrap_or_default().to_string();

        if field_name != "bot" {
            tracing::info!(field = %field_name, "skipping non-bot upload field");
            continue;
        }

        let data = field.bytes().await.map_err(|err| {
            error(
                StatusCode::BAD_REQUEST,
                format!("cannot read upload: {err}"),
            )
        })?;

        tracing::info!(
            email = %email,
            file_name = %file_name,
            size_bytes = data.len(),
            "bot file received (not stored)"
        );
    }

    Ok(Json(UploadResponse { ok: true }))
}

pub async fn my_bots(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<BotsResponse>, ApiError> {
    let email = session_email(&state, &jar)
        .ok_or_else(|| error(StatusCode::UNAUTHORIZED, "not logged in"))?;

    tracing::info!(email = %email, "listing bots");

    Ok(Json(BotsResponse {
        bots: vec![Bot {
            id: 1,
            name: "Settler".to_string(),
            language: "Python 3".to_string(),
            rank: 42,
            games_played: 1337,
            upload_date: "2026-11-01".to_string(),
        }],
    }))
}
