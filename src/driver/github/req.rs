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
    #[serde(rename = "title")]
    Title(String),

    #[serde(rename = "issue")]
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
    let url = format!("https://{base}/repos/{owner}/{repo}/pulls");

    let req = client
        .post(url)
        .header("User-Agent", "Tidal")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", token))
        .json(&req);

    let res = req.send().await?;
    if let Err(e) = res.error_for_status_ref() {
        log::error!("failed request: {}", res.text().await.unwrap());
        return Err(GithubError::Http(e));
    }

    let res: CreatePullRequestResponse = res.json().await?;

    Ok(res)
}
