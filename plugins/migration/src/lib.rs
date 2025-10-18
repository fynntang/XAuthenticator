use crate::sea_orm::DbConn;
use sea_orm_cli::MigrateSubcommands;
pub use sea_orm_migration::prelude::*;
use std::error::Error;

mod m20251011_150141_create_table_accounts;
mod m20251011_150151_create_table_meta;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251011_150141_create_table_accounts::Migration),
            Box::new(m20251011_150151_create_table_meta::Migration),
        ]
    }
}

pub async fn run_migrate(
    db: &DbConn,
    command: Option<MigrateSubcommands>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    cli::run_migrate(Migrator, db, command, verbose).await
}
