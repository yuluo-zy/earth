use std::cmp::min;
use std::fs::File;
use std::io::{Seek, Write};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use tracing::info;

pub mod config;
mod bing;
// mod bing;
// mod mock;
// mod pexels;


#[derive(Debug, Serialize, Deserialize)]
pub enum PhotoService {
    Bing,
    Pexels,
    Unsplash,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AsyncProcessMessage {
    StartRotate,
    StopRotate,
    PreviousPhoto,
    NextPhoto,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Images {
    pub bot: usize,
    pub copyright: String,
    pub copy_right_link: String,
    pub drk: usize,
    pub end_date: String,
    pub full_start_date: String,
    pub hs: Vec<String>,
    pub hsh: String,
    pub quiz: String,
    pub start_date: String,
    pub title: String,
    pub top: usize,
    pub url: String,
    pub urlbase: String,
    pub wp: bool,
}

pub trait ImagesTrait {

}




pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(anyhow!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(anyhow!("Failed to get content length from '{}'", &url))?;

    // let pb = ProgressBar::new(total_size);
    // pb.set_style(ProgressStyle::default_bar()
    //   .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
    //   .progress_chars("█  "));
    // pb.set_message(&format!("Downloading {}", url));

    let mut file;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    info!("Seeking in file.");

    if std::path::Path::new(path).exists() {
        info!("File exists. Resuming.");

        file = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)?;

        let file_size = std::fs::metadata(path)?.len();

        file.seek(std::io::SeekFrom::Start(file_size))?;
        downloaded = file_size;
    } else {
        info!("Fresh file..");

        file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    }

    file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;

    info!("Commencing transfer");

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(anyhow!("Error while downloading file")))?;

        file
            .write(&chunk)
            .or(Err(anyhow!("Error while writing to file")))?;
        downloaded = min(downloaded + (chunk.len() as u64), total_size);
        // pb.set_position(new);
    }

    // pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    info!("Downloaded ==> {:?} to {:?}", url, path);
    return Ok(path.to_string());
}

pub fn view_photo(handle: tauri::AppHandle, href: String) {
    let label = Images::get_filename(href.as_str());
    let label = "view_photo";

    info!("{:?}", label);

    let view_window = tauri::WindowBuilder::new(
        &handle,
        label,
        tauri::WindowUrl::External(href.parse().unwrap()),
    )
        .build()
        .unwrap();

    info!("{:?} ", href);
}
