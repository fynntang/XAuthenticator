use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Account {
    Table,
    Id, // UUID
    Issuer,
    Label,
    Type,      // TOTP | HOTP
    Algorithm, // SHA1 | SHA256 | SHA512
    Digits,
    Period,       // TOTP 专用（单位 秒）
    Counter,      // HOTP 专用
    SecretCipher, // AEAD ciphertext
    SecretNonce,  // nonce (XChaCha20-Poly1305 24 bytes)
    Icon,         // optional
    Note,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(uuid_uniq(Account::Id))
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
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}
