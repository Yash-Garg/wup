use std::path::PathBuf;

use color_eyre::eyre::Result;
use reqwest::header;
#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::{
    constants::{APP_USER_AGENT, INVALID_ARCH_OS, VALID_ARCH, VALID_OS},
    models::{
        config::RepoConfig,
        config::{CliConfig, VersionStore},
        github::{GithubRelease, GithubReleaseAsset},
    },
};

pub async fn start_update(config: CliConfig) -> Result<(), Box<dyn std::error::Error>> {
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
                            || asset_name.contains("win64")
                            || asset_name.contains("win-64")
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

        let new_vstore = VersionStore::new(release.clone(), &repo);
        match vstore {
            Some(v) => {
                let mut path = file_path.parent().unwrap().to_path_buf();
                path.push(&repo.name);

                println!(
                    "\nUpdating {} from {} to {}\n",
                    repo.name, v.tag, release.tag
                );
                asset.delete_dir(&path).unwrap_or_else(|_| {
                    panic!(
                        "Failed to delete old asset {} for {}/{}",
                        asset.name, repo.owner, repo.name
                    )
                });

                v.replace(new_vstore)?;
            }
            None => {
                new_vstore.write()?;
            }
        }

        let _final_path: Option<PathBuf> = if asset.name.ends_with(".zip") {
            Some(asset.extract(&file_path, &repo.name)?)
        } else if asset.name.ends_with(".tar.gz") {
            // TODO: Implement tar.gz extraction
            None
        } else if asset.name.ends_with(".exe") {
            Some(asset.move_dir(&file_path, &repo.name)?)
        } else {
            None
        };

        #[cfg(target_os = "windows")]
        match _final_path {
            Some(path) => {
                if cfg!(windows) {
                    println!("Adding {} entry to PATH.\n", &repo.name);

                    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                    let (env, _) = hkcu.create_subkey("Environment").unwrap();

                    let current_path = env
                        .get_value::<String, _>("PATH")
                        .unwrap_or_else(|_| "".to_string());

                    if current_path.is_empty() {
                        env.set_value("PATH", &path.to_str().unwrap())?;
                        return Ok(());
                    } else if current_path.contains(&asset.name) {
                        return Ok(());
                    } else {
                        let new_path = format!("{};{}", current_path, path.to_str().unwrap());
                        env.set_value("PATH", &new_path)?;
                    }
                }
            }
            None => {}
        }
    } else {
        eprintln!("No assets found for {}", repo.name);
    }

    Ok(())
}
