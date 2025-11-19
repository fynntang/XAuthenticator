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
pub struct InitRequest {
    pub kdbx_path: std::path::PathBuf,
    pub password: String,
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub current: u32,
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    pub issuer: String,
    pub label: String,
    #[serde(rename = "type")]
    pub r#type: String, // TOTP | HOTP
    pub algorithm: String, // SHA1 | SHA256 | SHA512
    pub digits: i32,
    pub period: Option<i32>,    // TOTP only (seconds)
    pub counter: Option<i32>,   // HOTP only
    pub secret: String,         // Base32 encoded secret
    pub icon: Option<Vec<u8>>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountRequest {
    pub id: String,
    pub issuer: Option<String>,
    pub label: Option<String>,
    pub icon: Option<Vec<u8>>,
    pub note: Option<String>,
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
        pub icon: Option<Vec<u8>>,
        pub note: Option<String>,
        pub created_at: Option<i64>,
        pub updated_at: Option<i64>,
    }
}
