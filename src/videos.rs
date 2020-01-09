use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::responses::*;
use crate::{TwitchApi, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixVideo {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub published_at: String,
    pub url: String,
    pub thumbnail_url: String,
    pub viewable: String,
    pub view_count: i32,
    pub language: String,
    pub r#type: String,
    pub duration: String
}

impl super::traits::HelixModel for HelixVideo {}

pub async fn get(twitch_api: &TwitchApi, video_ids: &Vec<String>) -> Result<HelixResponse<HelixVideo>> {
    let mut data: Vec<(&str, String)> = vec![];

    for video_id in video_ids {
        data.push(("id", String::from(video_id)));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/videos"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

pub async fn get_from_game(twitch_api: &TwitchApi, game_id: String, first: i32, after: Option<String>, before: Option<String>, language: Option<String>, period: Option<String>,
                           sort: Option<String>, r#type: Option<String>) -> Result<HelixPaginatedResponse<HelixVideo>> {
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

    if let Some(value) = language {
        data.push(("language", value));
    }

    if let Some(value) = period {
        data.push(("period", value));
    }

    if let Some(value) = sort {
        data.push(("sort", value));
    }

    if let Some(value) = r#type {
        data.push(("type", value));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/videos"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

pub async fn get_from_user(twitch_api: &TwitchApi, user_id: String, first: i32, after: Option<String>, before: Option<String>, language: Option<String>, period: Option<String>,
                           sort: Option<String>, r#type: Option<String>) -> Result<HelixPaginatedResponse<HelixVideo>> {
    let mut data: Vec<(&str, String)> = vec![
        ("user_id", String::from(&user_id[..])),
        ("first", first.to_string())
    ];

    if let Some(value) = after {
        data.push(("after", value));
    }

    if let Some(value) = before {
        data.push(("before", value));
    }

    if let Some(value) = language {
        data.push(("language", value));
    }

    if let Some(value) = period {
        data.push(("period", value));
    }

    if let Some(value) = sort {
        data.push(("sort", value));
    }

    if let Some(value) = r#type {
        data.push(("type", value));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/videos"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

