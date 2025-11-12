use std::path::PathBuf;

pub struct AppDataDir {
    app_data_dir: PathBuf,
}

impl AppDataDir {
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self { app_data_dir }
    }
    pub fn app(&self) -> PathBuf {
        self.app_data_dir.clone()
    }
    pub fn config(&self) -> PathBuf {
        self.app_data_dir.join("config.yaml")
    }
    pub fn version(&self) -> PathBuf {
        self.app_data_dir.join("version.txt")
    }
    pub fn accounts(&self) -> PathBuf {
        self.app_data_dir.join("accounts.kdbx")
    }
}
