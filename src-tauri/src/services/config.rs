use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Result};

const FILE_CONFIG: &str = "earth.toml";
const FILE_PATH: &str = ".earth";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarthConfig {
    pub auto_rotate: bool,
    pub rotate_source: Vec<String>,
    pub randomly: bool,
    pub interval_minute: u64,
    pub pexels_api_key: String
}

impl Default for EarthConfig {
    fn default() -> Self {
        Self {
            auto_rotate: false,
            rotate_source: vec![],
            randomly: false,
            interval_minute: 30,
            pexels_api_key: String::from(" ")
        }
    }
}

impl EarthConfig {
    pub fn create_app_folder() -> Result<()> {
        let home_dir = tauri::api::path::home_dir();

        match home_dir {
            Some(home_dir) => {
                let app_config_dir = Path::new(&home_dir).join(FILE_PATH);
                Ok(fs::create_dir_all(app_config_dir)?)
            }
            None => {
                Err(anyhow!("{}", "Home dir is not fount"))
            }
        }
    }

    pub fn get_app_folder() -> Result<PathBuf> {
        let home_dir = tauri::api::path::home_dir();

        match home_dir {
            Some(home_dir) => {
                let app_config_dir = Path::new(&home_dir).join(FILE_PATH);

                if !app_config_dir.exists() {
                    Self::create_app_folder()?
                }
                Ok(app_config_dir)
            }
            None => {
                Err(anyhow!("{}", "no home dir"))
            }
        }
    }

    pub fn write_config(&self) -> Result<()> {
        let folder_dir = Self::get_app_folder()?;
        let file_path = Path::new(&folder_dir).join(FILE_CONFIG);

        if !file_path.exists() {
            fs::File::create(&file_path).expect("create config failed");
        }

        let content = toml::to_string(self)?;
        Ok(fs::write(file_path, content)?)
    }

    pub fn get_config() -> Result<Self> {
        let folder_dir = Self::get_app_folder()?;
        let file_path = Path::new(&folder_dir).join(FILE_CONFIG);

        if !file_path.exists() {
            fs::File::create(&file_path).expect("create config failed");
        }

        let content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(_) => "".to_string(),
        };

        let data: Self = match toml::from_str(&content) {
            Ok(data) => Self { ..data },
            Err(_) => Self::default()
        };

        Ok(data)
    }

    pub fn set_auto_rotate(&mut self, auto_rotate: bool) {
        self.auto_rotate = auto_rotate;
        self.write_config().expect("write config error");
    }

    pub fn set_interval_minute(&mut self, interval: u64) {
        self.interval_minute = interval;
        self.write_config().expect("write config error")
    }

    // pub fn set_rotate_source(&self, source: String, checked: bool) -> Self {
    // let mut data = Self::get_config();
    //
    // if checked {
    //     data.rotate_source.push(source);
    // } else {
    //     data.rotate_source.retain(|x| *x != source);
    // }
    //
    // println!("data; {:?}", data);
    //
    // Self::write_config(data.clone());
    //
    // data
    // }

    pub fn set_randomly(&mut self, randomly: bool) {
        self.randomly = randomly;
        self.write_config().expect("write config error");
    }
}
