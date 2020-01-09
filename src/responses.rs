use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::traits::HelixModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixResponse<T: HelixModel> {
    pub data: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPaginatedResponse<T: HelixModel> {
    pub data: Vec<T>,
    pub pagination: HelixPagination
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPaginatedResponseWithTotal<T: HelixModel> {
    pub data: Vec<T>,
    pub pagination: HelixPagination,
    pub total: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixPagination {
    pub cursor: String
}
