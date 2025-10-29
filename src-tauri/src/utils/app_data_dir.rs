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
    pub fn db(&self, app_name: String) -> PathBuf {
        self.app_data_dir
            .join(format!("{}.db", app_name.to_lowercase()))
    }
    pub fn version(&self) -> PathBuf {
        self.app_data_dir.join("version.txt")
    }
    pub fn salt(&self) -> PathBuf {
        self.app_data_dir.join("salt.txt")
    }
    pub fn master_key(&self) -> PathBuf {
        self.app_data_dir.join("master.key")
    }
}
