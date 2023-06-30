use serde::{Deserialize, Serialize};

use crate::constants::{API_BASE_URL, VERSION_STORE};

use super::github::GithubRelease;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub repos: Vec<RepoConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoConfig {
    pub owner: String,
    pub name: String,
    pub force_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionStore {
    pub repo_name: String,
    pub release_id: u64,
    pub node_id: String,
    pub tag: String,
}

impl VersionStore {
    pub fn new(release: GithubRelease, repo: &RepoConfig) -> Self {
        Self {
            repo_name: format!("{}/{}", repo.owner, repo.name),
            release_id: release.release_id,
            node_id: release.node_id,
            tag: release.tag,
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut versions = VersionStore::read()?;
        versions.push(self.clone());

        let file = std::fs::File::create(VERSION_STORE)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &versions)?;

        Ok(())
    }

    pub fn read() -> Result<Vec<VersionStore>, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(VERSION_STORE).unwrap_or_else(|_| {
            std::fs::File::create(VERSION_STORE).unwrap_or_else(|_| {
                panic!(
                    "Failed to create {} file. Please make sure you have write permissions.",
                    VERSION_STORE
                )
            })
        });
        let reader = std::io::BufReader::new(file);
        let versions: Vec<VersionStore> =
            serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);
        Ok(versions)
    }

    pub fn get(repo_name: &str) -> Result<Option<VersionStore>, Box<dyn std::error::Error>> {
        let versions = VersionStore::read()?;
        let version = versions.into_iter().find(|v| v.repo_name == repo_name);
        Ok(version)
    }
}

impl RepoConfig {
    pub async fn fetch_latest_release(
        &self,
        client: &reqwest::Client,
    ) -> Result<GithubRelease, Box<dyn std::error::Error>> {
        let request_url = format!(
            "{}/repos/{}/{}/releases/{}",
            API_BASE_URL,
            self.owner,
            self.name,
            match &self.force_tag {
                Some(tag) => format!("tags/{}", tag),
                None => "latest".to_string(),
            }
        );

        let http_resp = client.get(&request_url).send().await?;
        let response: GithubRelease = http_resp.json().await?;

        return Ok(response);
    }
}
