use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::responses::*;
use crate::{TwitchApi, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixClip {
    pub id: String,
    pub url: String,
    pub embed_url: String,
    pub broadcaster_id: String,
    pub broadcaster_name: String,
    pub creator_id: String,
    pub creator_name: String,
    pub video_id: String,
    pub game_id: String,
    pub language: String,
    pub title: String,
    pub view_count: i32,
    pub created_at: String,
    pub thumbnail_url: String
}

impl super::traits::HelixModel for HelixClip {}

pub async fn get(twitch_api: &TwitchApi, clip_ids: &Vec<String>) -> Result<HelixResponse<HelixClip>> {
    let mut data: Vec<(&str, String)> = vec![];

    for clip_id in clip_ids {
        data.push(("id", String::from(clip_id)));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/clips"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

pub async fn get_from_game(twitch_api: &TwitchApi, game_id: String, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixPaginatedResponse<HelixClip>> {
    let mut data: Vec<(&str, String)> = vec![
        ("game_id", String::from(&game_id[..])),
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
            &twitch_api.get(String::from("https://api.twitch.tv/helix/clips"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

pub async fn get_from_broadcaster(twitch_api: &TwitchApi, broadcaster_id: String, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixPaginatedResponse<HelixClip>> {
    let mut data: Vec<(&str, String)> = vec![
        ("broadcaster_id", String::from(&broadcaster_id[..])),
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
            &twitch_api.get(String::from("https://api.twitch.tv/helix/clips"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}
