use std::cmp::min;
use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;
use anyhow::anyhow;
use anyhow::Result;
use reqwest::Client;
use tracing::info;
use futures_util::StreamExt;

pub async fn download_file(client: &Client, url: &str, path: &Path) -> Result<()> {
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

    if path.exists() {
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

        file = File::create(path).or(Err(anyhow!("Failed to create file '{:?}'", path)))?;
    }

    file = File::create(path).or(Err(anyhow!("Failed to create file '{:?}'", path)))?;

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

    return Ok(());
}
