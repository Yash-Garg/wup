#![allow(dead_code)]
mod constants;
mod models;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use reqwest::header;

use color_eyre::eyre::Result;
use constants::APP_USER_AGENT;

use crate::{
    constants::{INVALID_ARCH_OS, VALID_ARCH, VALID_OS},
    models::{config::CliConfig, github::GithubReleaseAsset},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let config: CliConfig = Figment::new()
        .merge(Yaml::file("config.yml"))
        .extract()
        .unwrap_or_else(|_| {
            panic!("Failed to load config.yml. Please make sure it exists and is valid YAML.");
        });

    dbg!(&config);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()?;

    for repo in &config.repos {
        let result = repo.fetch_latest_release(&client).await?;
        let mut filtered_assets = Vec::<GithubReleaseAsset>::new();

        for asset in result.assets {
            let asset_name = asset.name.to_lowercase();

            filtered_assets.extend(VALID_OS.iter().flat_map(|os| {
                VALID_ARCH.iter().flat_map(|arch| {
                    INVALID_ARCH_OS
                        .iter()
                        .filter(|key| {
                            asset_name.contains(*os)
                                && asset_name.contains(*arch)
                                && !asset_name.contains(*key)
                        })
                        .map(|_| asset.clone())
                })
            }));
        }

        if !filtered_assets.is_empty() {
            let asset = filtered_assets
                .iter()
                .find(|asset| asset.name.contains("msvc"))
                .unwrap_or_else(|| filtered_assets.first().unwrap());

            let file_path = asset.download(&client).await.unwrap_or_else(|_| {
                panic!(
                    "Failed to download asset {} for {}/{}",
                    asset.name, repo.owner, repo.name
                )
            });

            dbg!(file_path);
        } else {
            eprintln!("No assets found for {}", repo.name);
        }
    }

    Ok(())
}
