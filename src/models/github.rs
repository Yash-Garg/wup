use serde::Deserialize;

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
