#![allow(dead_code)]
mod constants;
mod models;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use models::{config::RepoConfig, github::GithubRelease};
use reqwest::header;

use color_eyre::eyre::Result;
use constants::APP_USER_AGENT;

use crate::{
    constants::{CLI_CONFIG, INVALID_ARCH_OS, VALID_ARCH, VALID_OS},
    models::{
        config::{CliConfig, VersionStore},
        github::GithubReleaseAsset,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let config: CliConfig = Figment::new()
        .merge(Yaml::file(CLI_CONFIG))
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

        let vstore = VersionStore::get(format!("{}/{}", repo.owner, repo.name).as_str())?;
        match vstore {
            Some(v) => {
                if v.tag != result.tag {
                    get_asset_and_store(&client, result, &repo, Some(v)).await?;
                }
            }

            None => {
                get_asset_and_store(&client, result, &repo, None).await?;
                continue;
            }
        }
    }

    Ok(())
}

async fn get_asset_and_store(
    client: &reqwest::Client,
    release: GithubRelease,
    repo: &RepoConfig,
    vstore: Option<VersionStore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let needs_update = match &vstore {
        Some(v) => {
            let is_tag_same = if repo.force_tag.is_some() {
                &v.tag == repo.force_tag.as_ref().unwrap()
            } else {
                false
            };

            v.tag != release.tag && !is_tag_same
        }
        None => true,
    };

    if !needs_update {
        println!(
            "Skipping {} as it is already up to date.",
            format!("{}/{}", repo.owner, repo.name)
        );
        return Ok(());
    }

    let mut filtered_assets = Vec::<GithubReleaseAsset>::new();

    for asset in &release.assets {
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

        match vstore {
            Some(v) => {
                let mut path = file_path.parent().unwrap().to_path_buf();
                path.push(&repo.name);

                dbg!(path.clone());

                println!("Updating {} from {} to {}", repo.name, v.tag, release.tag);
                asset.delete_dir(&path).unwrap_or_else(|_| {
                    panic!(
                        "Failed to delete old asset {} for {}/{}",
                        asset.name, repo.owner, repo.name
                    )
                });

                // TODO: replace in vstore with new tag and id
            }
            None => {
                let new_vstore = VersionStore::new(release, &repo);
                new_vstore.write()?;
            }
        }

        if asset.name.ends_with(".zip") {
            asset.extract(&file_path, &repo.name)?;
        } else if asset.name.ends_with(".exe") {
            asset.move_dir(&file_path, &repo.name)?;
        }
    } else {
        eprintln!("No assets found for {}", repo.name);
    }

    Ok(())
}
