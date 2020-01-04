pub mod models;
pub mod responses;
pub mod traits;

use self::{models::TwitchGame, responses::HelixResponse};

macro_rules! ret {
    ($returnType:ty) => {
        Result<$returnType, Box<dyn std::error::Error>>
    };
}

pub struct TwitchApi {
    client_id: String,
    client: reqwest::Client
}

impl TwitchApi {
    pub fn new(client_id: String) -> ret!(TwitchApi) {
        Ok(TwitchApi {
            client_id,
            client: reqwest::Client::builder().build()?
        })
    }

    pub async fn top_games(&self) -> ret!(HelixResponse<TwitchGame>) {
        Ok(
            serde_json::from_str(
                &self.get(String::from("https://api.twitch.tv/helix/games/top"))
                    .await?
                    .text()
                    .await?[..]
            )?
        )
    }

    async fn get(&self, url: String) -> ret!(reqwest::Response) {
        Ok(self.client.get(&url[..])
            .header("Client-ID", &self.client_id[..])
            .send()
            .await?)
    }
}
