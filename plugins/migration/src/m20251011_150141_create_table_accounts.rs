use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

// -- accounts: 存储每个 OTP 条目的 metadata 与加密 ciphertext
// CREATE TABLE IF NOT EXISTS accounts
// (
//     id            TEXT PRIMARY KEY, -- UUID
//     issuer        TEXT,
//     label         TEXT    NOT NULL,
//     type          TEXT    NOT NULL, -- 'TOTP' | 'HOTP'
//     algorithm     TEXT    NOT NULL, -- 'SHA1' | 'SHA256' | 'SHA512'
//     digits        INTEGER NOT NULL,
//     period        INTEGER,          -- TOTP 专用（单位 秒）
//     counter       INTEGER,          -- HOTP 专用
//     secret_cipher BLOB    NOT NULL, -- AEAD ciphertext
//     secret_nonce  BLOB    NOT NULL, -- nonce (XChaCha20-Poly1305 24 bytes)
//     created_at    INTEGER NOT NULL,
//     updated_at    INTEGER NOT NULL,
//     icon          BLOB,             -- optional
//     note          TEXT
// );

        

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id))
                    .col(string(Account::Issuer))
                    .col(string(Account::Label))
                    .col(string(Account::Type))
                    .col(string(Account::Algorithm))
                    .col(integer(Account::Digits))
                    .col(integer(Account::Period))
                    .col(integer(Account::Counter))
                    .col(binary(Account::SecretCipher))
                    .col(binary(Account::SecretNonce))
                    .col(integer(Account::CreatedAt))
                    .col(integer(Account::UpdatedAt))
                    .col(binary(Account::Icon))
                    .col(string(Account::Note))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Issuer,
    Label,
    Type,
    Algorithm,
    Digits,
    Period,
    Counter,
    SecretCipher,
    SecretNonce,
    Icon,
    Note,
    CreatedAt,
    UpdatedAt,
}
