use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TwitchGame {
    pub id: String,
    pub name: String,
    pub box_art_url: String
}

impl super::traits::TwitchModel for TwitchGame {}
