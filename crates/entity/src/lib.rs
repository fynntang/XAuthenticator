use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub issuer: Option<String>,
    pub account_name: Option<String>,
    pub secret: String,
    pub algorithm: String,
    pub digits: u32,
    pub period: u32,
    pub icon: Option<String>,
    pub notes: Option<String>,
    pub created_at: i64,
    pub modified_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    pub name: String,
    pub issuer: Option<String>,
    pub account_name: Option<String>,
    pub secret: String,
    pub algorithm: Option<String>,
    pub digits: Option<u32>,
    pub period: Option<u32>,
    pub icon: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountRequest {
    pub id: String,
    pub name: Option<String>,
    pub issuer: Option<String>,
    pub account_name: Option<String>,
    pub secret: Option<String>,
    pub algorithm: Option<String>,
    pub digits: Option<u32>,
    pub period: Option<u32>,
    pub icon: Option<String>,
    pub notes: Option<String>,
}

impl Default for Account {
    fn default() -> Self {
        let now = chrono::Local::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            issuer: None,
            account_name: None,
            secret: String::new(),
            algorithm: "SHA1".to_string(),
            digits: 6,
            period: 30,
            icon: None,
            notes: None,
            created_at: now,
            modified_at: now,
        }
    }
}
