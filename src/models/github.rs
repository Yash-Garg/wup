use serde::Deserialize;
use std::{cmp::min, fs::File, io::Write};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    #[serde(alias = "id")]
    pub release_id: u64,
    #[serde(alias = "tag_name")]
    pub tag: String,
    pub name: String,
    pub html_url: String,
    pub assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GithubReleaseAsset {
    #[serde(alias = "id")]
    pub asset_id: u64,
    pub node_id: String,
    pub name: String,
    pub size: u64,
    #[serde(alias = "browser_download_url")]
    pub download_url: String,
}

impl GithubReleaseAsset {
    pub async fn download(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let response = client
            .get(&self.download_url)
            .send()
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to download {}. Please make sure you have an internet connection.",
                    self.name
                );
            });

        let pb = ProgressBar::new(self.size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        ?.progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", self.name));

        if !std::path::Path::new("downloads").exists() {
            eprintln!("\"downloads\" directory does not exist. creating...");
            std::fs::create_dir("downloads")?;
        }

        let mut file = File::create(format!("downloads/{}", self.name))?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.unwrap_or_else(|_| {
                panic!(
                    "Failed to download {}. Please make sure you have an internet connection.",
                    self.name
                );
            });

            file.write_all(&chunk).unwrap_or_else(|_| {
                panic!(
                    "Failed to write {} to disk. Please make sure you have write permissions.",
                    self.name
                );
            });

            let new = min(downloaded + (chunk.len() as u64), self.size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!("Downloaded {}", self.name));
        Ok(())
    }
}
