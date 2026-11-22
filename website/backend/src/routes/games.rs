use axum::extract::Path;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct GameSummary {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize)]
pub struct GamesResponse {
    pub games: Vec<GameSummary>,
}

#[derive(Serialize)]
pub struct GameReplay {
    pub id: u64,
    #[serde(rename = "mapWidth")]
    pub map_width: u64,
    #[serde(rename = "mapHeight")]
    pub map_height: u64,
    pub players: Vec<String>,
    pub turns: u64,
}

pub async fn list() -> Json<GamesResponse> {
    tracing::info!("listing games");

    Json(GamesResponse {
        games: vec![
            GameSummary {
                id: 1,
                name: "Game #1".to_string(),
            },
            GameSummary {
                id: 2,
                name: "Game #2".to_string(),
            },
            GameSummary {
                id: 3,
                name: "Game #3".to_string(),
            },
        ],
    })
}

pub async fn get_game(Path(id): Path<u64>) -> Json<GameReplay> {
    tracing::info!(game_id = id, "fetching game replay metadata");

    Json(GameReplay {
        id,
        map_width: 240,
        map_height: 160,
        players: vec!["Settler".to_string(), "SettlerCopy".to_string()],
        turns: 300,
    })
}
