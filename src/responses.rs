use std::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixResponse<T: super::traits::HelixModel> {
    pub data: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPaginatedResponse<T: super::traits::HelixModel> {
    pub data: Vec<T>,
    pub pagination: super::models::HelixPagination
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPaginatedResponseWithTotal<T: super::traits::HelixModel> {
    pub data: Vec<T>,
    pub pagination: super::models::HelixPagination,
    pub total: i32
}
