use serde::{Deserialize, Serialize};

pub mod account;
pub mod meta;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDefault {
    pub kdbx_path: std::path::PathBuf,
}

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

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Account {
        pub id: String,
        pub issuer: String,
        pub label: String,
        #[serde(rename = "type")]
        pub r#type: String,
        pub algorithm: String,
        pub digits: i32,
        pub period: Option<i32>,
        pub counter: Option<i32>,
        pub secret_cipher: Vec<u8>,
    }
}
