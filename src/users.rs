use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::responses::*;
use crate::{TwitchApi, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixUser {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: i32,
    pub email: String
}

impl super::traits::HelixModel for HelixUser {}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixFollow {
    pub from_id: String,
    pub from_name: String,
    pub to_id: String,
    pub to_name: String,
    pub followed_at: String
}

impl super::traits::HelixModel for HelixFollow {}

pub async fn get(twitch_api: &TwitchApi, user_ids: &Vec<String>) -> Result<HelixResponse<HelixUser>> {
    let mut data: Vec<(&str, String)> = vec![];

    for user_id in user_ids {
        data.push(("id", String::from(user_id)));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/users"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}

pub async fn get_follows(twitch_api: &TwitchApi, first: i32, after: Option<String>, to_id: Option<String>, from_id: Option<String>) -> Result<HelixPaginatedResponseWithTotal<HelixFollow>> {
    let mut data: Vec<(&str, String)> = vec![
        ("first", first.to_string())
    ];

    if let Some(value) = after {
        data.push(("after", value));
    }

    if let Some(value) = to_id {
        data.push(("to_id", value));
    }

    if let Some(value) = from_id {
        data.push(("from_id", value));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/users/follows"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}
