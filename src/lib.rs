pub mod models;
pub mod responses;
pub mod traits;

use self::{models::HelixGame, responses::{HelixResponse, HelixPaginatedResponse} };

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! pagination {
    ($data:tt, $after:tt, $before:tt) => {
        if let Some(value) = $after {
            $data.push(("after", value));
        }

        if let Some(value) = $before {
            $data.push(("before", value));
        }
    }
}

pub struct TwitchApi {
    client_id: String,
    client: reqwest::Client
}

impl TwitchApi {
    pub fn new(client_id: String) -> Result<TwitchApi> {
        Ok(TwitchApi {
            client_id,
            client: reqwest::Client::builder().build()?
        })
    }

    pub async fn top_games(&self, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixPaginatedResponse<HelixGame>> {
        let mut data: Vec<(&str, String)> = vec![("first", first.to_string())];

        pagination!(data, after, before);

        Ok(
            serde_json::from_str(
                &self.get(String::from("https://api.twitch.tv/helix/games/top"), &data)
                    .await?
                    .text()
                    .await?[..]
            )?
        )
    }

    pub async fn games(&self, game_ids: &Vec<String>) -> Result<HelixResponse<HelixGame>> {
        let mut data: Vec<(&str, String)> = vec![];

        for game_id in game_ids {
            data.push(("id", String::from(game_id)));
        }

        Ok(
            serde_json::from_str(
                &self.get(String::from("https://api.twitch.tv/helix/games"), &data)
                    .await?
                    .text()
                    .await?[..]
            )?
        )
    }

    async fn get(&self, url: String, data: &Vec<(&str, String)>) -> Result<reqwest::Response> {
        Ok(self.client.get(&url[..])
            .header("Client-ID", &self.client_id[..])
            .query(&data)
            .send()
            .await?)
    }
}
