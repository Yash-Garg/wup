use serde::Deserialize;
use std::{
    cmp::min,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

#[derive(Deserialize, Debug, Clone)]
pub struct GithubRelease {
    #[serde(alias = "id")]
    pub release_id: u64,
    pub node_id: String,
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
    pub async fn download(&self, client: &Client) -> Result<PathBuf, Box<dyn std::error::Error>> {
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

        if !Path::new("downloads").exists() {
            eprintln!("\"downloads\" directory does not exist. creating...");
            std::fs::create_dir("downloads")?;
        }

        let path = Path::new("downloads").join(&self.name);
        let mut file = File::create(&path)?;
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
        Ok(fs::canonicalize(&path).unwrap_or_else(|_| path))
    }

    pub fn extract(
        &self,
        path: &PathBuf,
        folder_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Extracting {}...", &self.name);

        let zipfile = std::fs::File::open(&path).unwrap();

        let mut archive = zip::ZipArchive::new(zipfile)
            .unwrap_or_else(|_| panic!("Failed to open {}.", &self.name));

        let mut subfolder = false;
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            if file.is_dir() {
                subfolder = true;
                break;
            }
        }

        let mut extraction_path = path.parent().unwrap().to_path_buf();
        if !subfolder {
            extraction_path.push(folder_name);
        }

        archive
            .extract(&extraction_path.to_str().unwrap())
            .unwrap_or_else(|_| panic!("Failed to extract archive {}.", &self.name));

        println!("Extracted {} to {:#?}.\n", &self.name, extraction_path);

        let mut extracted_path = extraction_path.clone();
        extracted_path.push(&self.name[..self.name.len() - 4]);

        let mut subfolder_path = extraction_path.clone();
        subfolder_path.push(&folder_name);

        if subfolder {
            std::fs::rename(&extracted_path, &subfolder_path).unwrap_or_else(|_| {
                panic!(
                    "Failed to rename {} to {}.",
                    extracted_path.to_str().unwrap(),
                    subfolder_path.to_str().unwrap()
                );
            });
        }

        std::fs::remove_file(&path).unwrap_or_else(|_| {
            panic!("Failed to remove {}.", path.to_str().unwrap());
        });

        Ok(())
    }

    pub fn move_dir(
        &self,
        path: &PathBuf,
        folder_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut new_path = path.clone();
        new_path.pop();
        new_path.push(&folder_name);
        new_path.push(&self.name);

        std::fs::create_dir_all(new_path.parent().unwrap()).unwrap_or_else(|_| {
            panic!(
                "Failed to create directory {}.",
                new_path.parent().unwrap().to_str().unwrap()
            );
        });

        std::fs::rename(&path, &new_path).unwrap_or_else(|_| {
            panic!(
                "Failed to move {} to {}.",
                path.to_str().unwrap(),
                new_path.to_str().unwrap()
            );
        });

        Ok(())
    }

    pub fn delete_dir(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_dir_all(&path).unwrap_or_else(|_| {
            panic!("Failed to remove directory {}.", path.to_str().unwrap());
        });

        Ok(())
    }
}
