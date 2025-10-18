use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Default, Serialize, Clone)]
pub struct AppState {
    pub is_initialized: bool,
    pub config: xauthenticator_config::Config,
    #[serde(skip)]
    pub db: Option<DatabaseConnection>,
    pub is_locked: bool,
}

impl AppState {
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
    pub fn db(&self) -> Option<&DatabaseConnection> {
        self.db.as_ref()
    }
    pub fn config(&self) -> &xauthenticator_config::Config {
        &self.config
    }
}
