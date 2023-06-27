use serde::{Deserialize, Serialize};

use crate::constants::API_BASE_URL;

use super::github::GithubRelease;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub repos: Vec<RepoConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    pub owner: String,
    pub name: String,
    pub force_tag: Option<String>,
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
