use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use super::{error, ApiError, AppState};

#[derive(Serialize)]
pub struct StarterKit {
    pub name: String,
    pub language: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
}

#[derive(Serialize)]
pub struct StarterKitsResponse {
    pub starterkits: Vec<StarterKit>,
}

/// Enumerates the airesources directory live on every request.
pub async fn list(State(state): State<AppState>) -> Result<Json<StarterKitsResponse>, ApiError> {
    tracing::info!(directory = %state.airesources.display(), "listing starterkits");

    let mut entries = tokio::fs::read_dir(&state.airesources)
        .await
        .map_err(|err| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot read airesources directory: {err}"),
            )
        })?;

    let mut starterkits = Vec::new();
    while let Some(entry) = entries.next_entry().await.map_err(|err| {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("cannot read airesources entry: {err}"),
        )
    })? {
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || matches!(name.as_str(), "README.MD" | "starterkitdocs") {
            continue;
        }

        let file_type = entry.file_type().await.map_err(|err| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot inspect airesources entry: {err}"),
            )
        })?;
        if !file_type.is_dir() {
            continue;
        }

        starterkits.push(StarterKit {
            language: pretty_language(&name),
            download_url: format!("/api/starterkits/{name}/download"),
            name,
        });
    }

    starterkits.sort_by(|a, b| a.name.cmp(&b.name));
    tracing::info!(count = starterkits.len(), "starterkits listed");

    Ok(Json(StarterKitsResponse { starterkits }))
}

pub async fn download(Path(name): Path<String>) -> ApiError {
    tracing::info!(starterkit = %name, "starterkit download requested");

    error(StatusCode::NOT_IMPLEMENTED, "downloads not implemented yet")
}

fn pretty_language(directory: &str) -> String {
    match directory {
        "ML-StarterBot-Python" => return "Python 3 (ML)".to_string(),
        "CSharp" => return "C#".to_string(),
        "FSharp" => return "F#".to_string(),
        _ => {}
    }

    // "Python3" -> "Python 3", "Cython3" -> "Cython 3".
    if let Some(last) = directory.chars().last() {
        if last.is_ascii_digit() {
            let prefix = &directory[..directory.len() - last.len_utf8()];
            return format!("{prefix} {last}");
        }
    }

    directory.to_string()
}
