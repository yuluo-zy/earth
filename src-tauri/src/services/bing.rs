use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::download_file;
use anyhow::Result;
use tracing::{debug, info};
use crate::services::config;

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
  pub fn url(&self) -> String {
    ["https://www.bing.com", &self.url].concat()
  }

  pub fn get_filename(url: &str) -> Result<&str> {
    let s = url.find("OHR.").ok_or(0)?;
    let e = url.find("&rf=").ok_or(0)?;

    Ok(&url[s..e])
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveResources {
  pub images: Vec<Images>,
}

impl PrimitiveResources {
  pub async fn get_resources(index: u8, number: u8) -> Result<Self> {
    Ok(
      reqwest::get(get_url(index, number))
        .await?
        .json::<Self>()
        .await?,
    )
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallpaper {
  index: u8,
  number: u8,
  files: Vec<String>,
  pub json: PrimitiveResources,
}

impl Wallpaper {
  pub async fn new(index: u8, number: u8) -> Result<Wallpaper> {
    let json = PrimitiveResources::get_resources(index, number).await?;
    Ok(Wallpaper {
      index,
      number,
      files: vec![],
      json,
    })
  }

  pub async fn save_wallpaper(url: &str, filename: Option<&str>) -> Result<String> {
    // todo 修改为 并发下载
    let filename = match filename {
      Some(filename) => filename,
      None => Images::get_filename(url),
    };
    let app_folder = config::EarthConfig::get_app_folder()?;
    let path = app_folder.join(filename);
    let res = download_file(&Client::new(), &url, path.to_str()?).await?;

    info!("{:?}", res);
    Ok(res)
  }

  pub async fn set_wallpaper_from_local(a: String) -> String {
    wallpaper::set_from_path(a.as_str()).unwrap();

    if cfg!(not(target_os = "macos")) {
      wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    }

    a
  }

  pub async fn set_wallpaper(url: &str) -> Result<String> {
    let a = Wallpaper::save_wallpaper(url, None).await;
    match a {
      Ok(a) => {
        Self::set_wallpaper_from_local(a).await;

        Ok(String::from("OK"))
      }
      Err(e) => Err(e.to_string().into()),
    }
  }
}


