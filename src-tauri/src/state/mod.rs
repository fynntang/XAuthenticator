use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Default, Serialize, Clone)]
pub struct AppState {
    pub is_initialized: bool,
    pub runtime_duration: u64,
    pub config: xauthenticator_config::Config,
    #[serde(skip)]
    pub db: Option<DatabaseConnection>,
    pub is_locked: bool,
    #[serde(skip)]
    pub master_key: Option<[u8; 32]>,
}

impl AppState {}
