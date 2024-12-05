use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppState {
    pub html: String,
    pub preset_matrixes: Vec<GolPreset>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GolPreset {
    pub display_name: String,
    pub id: String,
    pub matrix: Vec<Vec<u8>>,
}

#[derive(Deserialize)]
pub struct CheckMatrixRequest(pub Vec<Vec<u8>>);
