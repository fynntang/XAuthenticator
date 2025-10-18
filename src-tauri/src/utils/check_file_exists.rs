use crate::utils::app_data_dir::AppDataDir;
use std::path::PathBuf;

pub struct CheckFileExists {
    dir: AppDataDir,
}

impl CheckFileExists {
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self {
            dir: AppDataDir::new(app_data_dir),
        }
    }
    pub fn app(&self) -> bool {
        self.dir.app().exists()
    }
    pub fn config(&self) -> bool {
        self.dir.config().exists()
    }
    pub fn db(&self, app_name: String) -> bool {
        self.dir.db(app_name).exists()
    }
    pub fn version(&self) -> bool {
        self.dir.version().exists()
    }

    pub fn all(&self, app_name: String) -> bool {
        self.app() && self.config() && self.db(app_name) && self.version()
    }
}
