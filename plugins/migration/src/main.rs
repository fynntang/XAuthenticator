use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(xauthenticator_migration::Migrator).await;
}
