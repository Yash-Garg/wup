#![allow(dead_code)]
mod constants;
mod models;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use models::github::GithubRepo;
use reqwest::header;

use color_eyre::eyre::Result;
use constants::APP_USER_AGENT;

use crate::models::config::CliConfig;

fn main() -> Result<()> {
    color_eyre::install()?;

    let config: CliConfig = Figment::new()
        .merge(Yaml::file("config.yml"))
        .extract()
        .unwrap_or_else(|_| {
            panic!("Failed to load config.yml. Please make sure it exists and is valid YAML.");
        });

    dbg!(config);

    Ok(())
}

async fn test_fetching_release() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()?;

    let repo = GithubRepo {
        owner: "sharkdp".to_string(),
        name: "fd".to_string(),
    };

    let result = repo.fetch_latest_release(&client).await?;

    for asset in result.assets {
        println!("{}", asset.name);
    }

    return Ok(());
}
