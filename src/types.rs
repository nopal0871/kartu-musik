use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ExportResolution {
    FullHD, // 1080p
    TwoK,   // 1440p
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardData {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
    pub audio_url: String,
    pub orientation: Orientation,
    pub template_id: String,
}

impl Default for CardData {
    fn default() -> Self {
        Self {
            title: "Simpatik Musik".to_string(),
            artist: "NDX A.K.A".to_string(),
            cover_url: "https://via.placeholder.com/300".to_string(),
            audio_url: "".to_string(),
            orientation: Orientation::Portrait,
            template_id: "ios_glass".to_string(),
        }
    }
}
