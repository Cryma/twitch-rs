use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HelixResponse<T: super::traits::TwitchModel> {
    pub data: Vec<T>
}
