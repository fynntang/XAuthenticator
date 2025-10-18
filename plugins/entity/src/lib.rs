use serde::{Deserialize, Serialize};

pub mod account;
pub mod meta;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    pub current: u32,
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub data: T,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CursorPageParam {
    pub page_size: u32,
    pub cursor: String,
    pub create_id: Option<String>,
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CursorPageResp<T> {
    pub cursor: String,
    pub is_last: bool,
    pub list: Option<T>,
    pub total: u64,
}
