mod constants;
mod models;

use models::github::GithubRepo;
use reqwest::header;

use constants::APP_USER_AGENT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
