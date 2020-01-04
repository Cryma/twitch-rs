use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixResponse<T: super::traits::TwitchModel> {
    pub data: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPaginatedResponse<T: super::traits::TwitchModel> {
    pub data: Vec<T>,
    pub pagination: super::models::TwitchPagination
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPaginatedResponseWithTotal<T: super::traits::TwitchModel> {
    pub data: Vec<T>,
    pub pagination: super::models::TwitchPagination,
    pub total: i32
}
