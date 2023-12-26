use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginatedParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResult<T: Serialize> {
    pub results: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

pub mod config;
