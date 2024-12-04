use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct InputData(pub Vec<Vec<u8>>);
