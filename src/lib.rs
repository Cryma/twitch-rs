pub mod models;
pub mod responses;
pub mod traits;

use self::{models::TwitchGame, responses::HelixResponse};
use std::error::Error;

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

    pub async fn top_games(&self) -> Result<HelixResponse<TwitchGame>, Box<dyn Error>> {
        Ok(
            serde_json::from_str(
                &self.get(String::from("https://api.twitch.tv/helix/games/top"))
                    .await?
                    .text()
                    .await?[..]
            )?
        )
    }

    async fn get(&self, url: String) -> Result<reqwest::Response, Box<dyn Error>> {
        Ok(self.client.get(&url[..])
            .header("Client-ID", &self.client_id[..])
            .send()
            .await?)
    }
}
