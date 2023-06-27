mod constants;
mod models;

use models::github::{GithubRelease, GithubRepo};
use reqwest::header;

use constants::{API_BASE_URL, APP_USER_AGENT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let repo = GithubRepo {
        owner: "sharkdp".to_string(),
        name: "fd".to_string(),
    };

    let request_url = format!(
        "{}/repos/{}/{}/releases/latest",
        API_BASE_URL, repo.owner, repo.name
    );

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()?;

    let http_resp = client.get(&request_url).send().await?;
    let response: GithubRelease = http_resp.json().await?;

    println!("{:#?}", response);

    return Ok(());
}
