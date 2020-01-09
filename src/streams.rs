use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::responses::*;
use crate::{TwitchApi, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixStream {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub game_id: String,
    pub r#type: String,
    pub title: String,
    pub viewer_count: i32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String
}

impl super::traits::HelixModel for HelixStream {}

pub async fn get_from_games(twitch_api: &TwitchApi, game_ids: &Vec<String>, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixPaginatedResponse<HelixStream>> {
    let mut data: Vec<(&str, String)> = vec![
        ("first", first.to_string())
    ];

    for game_id in game_ids {
        data.push(("game_id", String::from(game_id)));
    }

    if let Some(value) = after {
        data.push(("after", value));
    }

    if let Some(value) = before {
        data.push(("before", value));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/streams"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

pub async fn get_from_users(twitch_api: &TwitchApi, user_ids: &Vec<String>, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixPaginatedResponse<HelixStream>> {
    let mut data: Vec<(&str, String)> = vec![
        ("first", first.to_string())
    ];

    for user_id in user_ids {
        data.push(("user_id", String::from(user_id)));
    }

    if let Some(value) = after {
        data.push(("after", value));
    }

    if let Some(value) = before {
        data.push(("before", value));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/streams"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

