use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::GithubError;

#[derive(Serialize, Debug)]
pub struct CreatePullRequest {
    #[serde(flatten)]
    pub id: Id,
    pub head: String,
    #[serde(skip)]
    pub head_repo: Option<String>,
    pub base: String,
    pub body: String,
    pub draft: bool,
    pub maintainer_can_modify: bool,
}

#[derive(Serialize, Debug)]
pub enum Id {
    Title(String),
    Issue(u64),
}

#[derive(Deserialize)]
pub struct CreatePullRequestResponse {
    pub url: String,
}

pub async fn send_request(
    client: &Client,
    base: &str,
    owner: &str,
    repo: &str,
    token: &str,
    req: CreatePullRequest,
) -> Result<CreatePullRequestResponse, GithubError> {
    let url = format!("https://{base}/repos/{owner}/{repo}");

    let req = client
        .post(url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", token))
        .json(&req);

    let res = req.send().await?;
    let res = res.error_for_status()?;
    let res: CreatePullRequestResponse = res.json().await?;

    Ok(res)
}
