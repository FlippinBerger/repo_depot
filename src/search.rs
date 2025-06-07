use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Repo {
    pub name: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<Repo>,
}

pub async fn search(query: &str) -> Result<Vec<Repo>, Box<dyn std::error::Error>> {
    let client = Client::builder().user_agent("repo-depot").build()?;

    let response = client
        .get("https://api.github.com/search/repositories")
        .query(&[("q", query)])
        .send()
        .await?;
    let body: SearchResult = response.json().await?;

    Ok(body.items)
}
