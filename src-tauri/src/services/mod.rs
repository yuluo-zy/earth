use std::cmp::min;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tracing::info;

pub mod config;
pub mod bing;
pub mod storage;
pub mod cmd;
pub mod earth;


#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum PhotoService {
    BingDaily,
    BingList,
    Pexels,
    Unsplash,
    Earth,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperInfo {
    pub link: String,
    pub title: String,
    pub content: String,
}

#[async_trait]
pub trait WallpaperTrait: Send+ Sync {
    async fn set_wallpaper(&self) -> Result<()>;
    async fn save_wallpaper(&self) -> Result<PathBuf>;
    fn get_wallpaper_info(&self) -> Result<PaperInfo>;
}


// pub fn view_photo(handle: tauri::AppHandle, href: String) {
//     let label = Images::get_filename(href.as_str());
//     let label = "view_photo";
//
//     info!("{:?}", label);
//
//     let view_window = tauri::WindowBuilder::new(
//         &handle,
//         label,
//         tauri::WindowUrl::External(href.parse().unwrap()),
//     )
//         .build()
//         .unwrap();
//
//     info!("{:?} ", href);
// }
