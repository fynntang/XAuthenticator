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
    pub fn version(&self) -> bool {
        self.dir.version().exists()
    }
    pub fn accounts(&self) -> bool {
        self.dir.accounts().exists()
    }

    pub fn all(&self) -> bool {
        self.app() && self.config() && self.accounts() && self.version()
    }
}
