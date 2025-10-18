use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "account")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(default)]
    pub id: String, // UUID
    pub issuer: String,
    pub label: String,
    #[serde(rename = "type")]
    pub type_: String, // TOTP | HOTP
    pub algorithm: String, // SHA1 | SHA256 | SHA512
    pub digits: i32,
    pub period: Option<i32>,    // TOTP 专用（单位 秒）
    pub counter: Option<i32>,   // HOTP 专用
    pub secret_cipher: Vec<u8>, // AEAD ciphertext
    pub secret_nonce: Vec<u8>,  // nonce (XChaCha20-Poly1305 24 bytes)
    pub icon: Option<Vec<u8>>,
    pub note: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
