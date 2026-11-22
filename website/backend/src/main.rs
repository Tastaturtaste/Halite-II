mod routes;
mod session;

use std::path::PathBuf;

use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use routes::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let airesources = resolve_directory(
        &["airesources", "../airesources", "../../airesources"],
        "airesources",
    );
    // Covers starting from website/backend, website/, and the repo root
    // (the latter is how `mise run backend` launches the binary).
    let frontend_dist = resolve_directory(
        &[
            "../frontend/dist",
            "frontend/dist",
            "website/frontend/dist",
        ],
        "frontend dist",
    );

    tracing::info!(path = %airesources.display(), "using airesources directory");
    tracing::info!(path = %frontend_dist.display(), "using frontend dist directory");

    let state = AppState {
        sessions: session::new_store(),
        airesources,
    };

    let static_files =
        ServeDir::new(&frontend_dist).fallback(ServeFile::new(frontend_dist.join("index.html")));

    let app = routes::router(state).fallback_service(static_files).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind 0.0.0.0:3000");

    tracing::info!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("server failed");
}

/// Probes candidates in order and returns the first existing directory.
/// Falls back to the first candidate so the app can still start.
fn resolve_directory(candidates: &[&str], description: &str) -> PathBuf {
    for candidate in candidates {
        let path = PathBuf::from(candidate);
        if path.is_dir() {
            return path;
        }
    }

    tracing::warn!(
        candidates = ?candidates,
        "no {description} directory found; using first candidate"
    );
    PathBuf::from(candidates[0])
}
