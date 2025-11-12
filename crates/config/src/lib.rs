use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub theme: String,
    pub language: String,
    pub auto_lock: bool,
    pub auto_lock_timeout: u64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Builder {
    pub kdbx_path: PathBuf,
    pub settings: Settings,
}

impl Builder {
    pub fn set_kdbx_path(mut self, p: PathBuf) -> Self {
        self.kdbx_path = p;
        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            kdbx_path: PathBuf::new(),
            settings: Settings {
                theme: "system".to_string(),
                language: "en".to_string(),
                auto_lock: false,
                auto_lock_timeout: 10,
            },
        }
    }
}

impl Builder {
    pub fn settings(&mut self, settings: Settings) -> &mut Self {
        self.settings = settings;
        self
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    path: PathBuf,
    builder: Builder,
}

impl Config {
    pub fn init(path: PathBuf) -> Self {
        let cfg = Self {
            path,
            builder: Builder::default(),
        };

        if !cfg.path.exists() {
            cfg.store();
        };

        cfg
    }

    pub fn load(&mut self) -> Self {
        let data = fs::read_to_string(self.path.clone()).expect("failed to read config file");
        self.builder = serde_yaml::from_str(&data).expect("failed to parse config file");
        self.clone()
    }
    pub fn store(&self) {
        let data = serde_yaml::to_string(&self.builder).expect("failed to serialize config file");
        fs::write(self.path.clone(), data).expect("failed to write config file");
    }

    pub fn set_path(&mut self, path: PathBuf) -> &mut Self {
        self.path = path;
        self
    }
    pub fn set_builder(&mut self, builder: Builder) -> &mut Self {
        self.builder = builder;
        self
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn builder(&self) -> &Builder {
        &self.builder
    }
}
