use serde::Deserialize;

use crate::constants::API_BASE_URL;

#[derive(Deserialize, Debug)]
pub struct GithubRepo {
    pub owner: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub html_url: String,
    pub assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub browser_download_url: String,
}

impl GithubRepo {
    pub async fn fetch_latest_release(
        &self,
        client: &reqwest::Client,
    ) -> Result<GithubRelease, Box<dyn std::error::Error>> {
        let request_url = format!(
            "{}/repos/{}/{}/releases/latest",
            API_BASE_URL, self.owner, self.name
        );

        let http_resp = client.get(&request_url).send().await?;
        let response: GithubRelease = http_resp.json().await?;

        return Ok(response);
    }
}
