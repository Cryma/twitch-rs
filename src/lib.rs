pub mod models;
pub mod responses;
pub mod traits;

use self::{models::TwitchGame, responses::HelixResponse};
use std::error::Error;
use std::collections::HashMap;

pub struct TwitchApi {
    client_id: String,
    client: reqwest::Client
}

impl TwitchApi {
    pub fn new(client_id: String) -> Result<TwitchApi, Box<dyn Error>> {
        Ok(TwitchApi {
            client_id,
            client: reqwest::Client::builder().build()?
        })
    }

    pub async fn top_games(&self, first: i32, after: Option<String>, before: Option<String>) -> Result<HelixResponse<TwitchGame>, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("first", first.to_string());

        if let Some(value) = after {
            data.insert("after", value);
        }

        if let Some(value) = before {
            data.insert("before", value);
        }

        Ok(
            serde_json::from_str(
                &self.get(String::from("https://api.twitch.tv/helix/games/top"), &data)
                    .await?
                    .text()
                    .await?[..]
            )?
        )
    }

    async fn get(&self, url: String, data: &HashMap<&str, String>) -> Result<reqwest::Response, Box<dyn Error>> {
        Ok(self.client.get(&url[..])
            .header("Client-ID", &self.client_id[..])
            .query(&data)
            .send()
            .await?)
    }
}
