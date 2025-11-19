use keepass::Database;
use serde::Serialize;

#[derive(Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub is_initialized: bool,
    pub runtime_timestamp: u64,
    pub is_locked: bool,
    pub locked_timestamp: Option<u64>,
    pub config: xauthenticator_config::Config,

    #[serde(skip)]
    pub db: Option<Database>,
    
    #[serde(skip)]
    pub master_password: Option<String>,
}

impl AppState {}
