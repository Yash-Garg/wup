use serde::{Deserialize, Serialize};

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
