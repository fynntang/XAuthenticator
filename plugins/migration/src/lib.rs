pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
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
