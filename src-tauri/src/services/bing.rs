use std::path::{Path, PathBuf};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tracing::{debug, info};
use crate::plugins::utils::download_file;
use crate::services::{config, ImagesTrait, WallpaperTrait};

const BING_URL: &str = "https://www.bing.com/HPImageArchive.aspx?&format=js&nc=1612409408851&pid=hp&FORM=BEHPTB&uhd=1&uhdwidth=3840&uhdheight=2160";

fn get_url(index: u8, number: u8) -> String {
    let url = [BING_URL, "&idx=", &index.to_string(), "&n=", &number.to_string(), ].join("");
    debug!("url: {:?}", url);
    url
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Images {
    pub startdate: String,
    pub fullstartdate: String,
    pub enddate: String,
    pub url: String,
    pub urlbase: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub title: String,
    pub quiz: String,
    pub wp: bool,
    pub hsh: String,
    pub drk: usize,
    pub top: usize,
    pub bot: usize,
    pub hs: Vec<String>,
}

impl Images {
    pub fn hosts(&self) -> String {
        ["https://www.bing.com", &self.url].concat()
    }

    pub fn get_filename(&self) -> Result<&str> {
        let s = self.url.find("OHR.").ok_or(anyhow!("not find fileName"))?;
        let e = self.url.find("&rf=").ok_or(anyhow!("not find fileName"))?;

        Ok(&self.url[s..e])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveResources {
    index: Option<u8>,
    number: Option<u8>,
    pub images: Vec<Images>,
}

impl PrimitiveResources {
    pub async fn get_resources(index: u8, number: u8) -> Result<Self> {
        let mut res = reqwest::get(get_url(index, number)).await?.json::<Self>().await?;
        res.index = Some(index);
        res.number = Some(number);
        Ok(res)
    }
}

#[async_trait]
impl WallpaperTrait for Images {
    async fn set_wallpaper(&self) -> Result<()> {
        let path = self.save_wallpaper().await?;
        wallpaper::set_from_path(path.to_str().ok_or(anyhow!("path get error"))?).expect("set wallpaper error");
        if cfg!(not(target_os = "macos")) {
            wallpaper::set_mode(wallpaper::Mode::Crop).expect("set wallpaper mode error");
        }
        Ok(())
    }

    async fn save_wallpaper(&self) -> Result<PathBuf> {
        let filename = self.get_filename()?;
        let app_folder = config::EarthConfig::get_app_folder()?;
        let path = app_folder.join(filename);
        download_file(&Client::new(), self.hosts().as_str(), path.as_path()).await?;
        Ok(path)
    }
}


