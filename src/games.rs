use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::responses::*;
use crate::{TwitchApi, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixGame {
    pub id: String,
    pub name: String,
    pub box_art_url: String
}

impl super::traits::HelixModel for HelixGame {}

pub async fn get_top(twitch_api: &TwitchApi, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixPaginatedResponse<HelixGame>> {
    let mut data: Vec<(&str, String)> = vec![
        ("first", first.to_string())
    ];

    if let Some(value) = after {
        data.push(("after", value));
    }

    if let Some(value) = before {
        data.push(("before", value));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/games/top"), &data)
            .await?
            .text()
            .await?[..]
        )?
    )
}

pub async fn get(twitch_api: &TwitchApi, game_ids: &Vec<String>) -> Result<HelixResponse<HelixGame>> {
    let mut data: Vec<(&str, String)> = vec![];

    for game_id in game_ids {
        data.push(("id", String::from(game_id)));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/games"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}
