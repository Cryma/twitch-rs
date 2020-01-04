use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchClip {
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

impl super::traits::TwitchModel for TwitchClip {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchFollow {
    pub from_id: String,
    pub from_name: String,
    pub to_id: String,
    pub to_name: String,
    pub followed_at: String
}

impl super::traits::TwitchModel for TwitchFollow {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchGame {
    pub id: String,
    pub name: String,
    pub box_art_url: String
}

impl super::traits::TwitchModel for TwitchGame {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchPagination {
    pub cursor: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchStream {
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

impl super::traits::TwitchModel for TwitchStream {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchUser {
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

impl super::traits::TwitchModel for TwitchUser {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchVideo {
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

impl super::traits::TwitchModel for TwitchVideo {}
