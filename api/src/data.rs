use serde::{Deserialize, Serialize};

pub struct AppState {
    pub html: String,
}

#[derive(Deserialize)]
pub struct GenerateRandom {
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize)]
pub struct CheckGameBoardOptions {
    pub use_toroidal: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GolPreset {
    pub display_name: String,
    pub id: String,
    pub matrix: Vec<Vec<u8>>,
}

#[derive(Deserialize)]
pub struct InputData(pub Vec<Vec<u8>>);
