pub mod responses;
pub mod traits;

pub mod games;
pub mod clips;
pub mod streams;
pub mod users;
pub mod videos;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    async fn get(&self, url: String, data: &Vec<(&str, String)>) -> Result<reqwest::Response> {
        Ok(self.client.get(&url[..])
            .header("Client-ID", &self.client_id[..])
            .query(&data)
            .send()
            .await?)
    }
}
