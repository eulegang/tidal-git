use std::path::PathBuf;

use super::{DriverError, Runner};
use crate::cred::find_token;
use crate::driver::github::cli::GithubCli;
use crate::driver::github::req::CreatePullRequest;
use crate::errors::SysErrorHandler;
use crate::tidal::Req;
use clap::Parser;

pub use err::GithubError;
use gix::remote::Direction;
use gix::Repository;

mod cli;
mod err;
mod req;

/// Create a pull request with githubs [pull request api](https://docs.github.com/en/free-pro-team@latest/rest/pulls/pulls?apiVersion=2022-11-28#create-a-pull-request)
pub struct Github {
    /// Base use
    pub base: String,
}

impl Github {
    async fn create_pull_request(self, repo: Repository, mut req: Req) -> Result<(), GithubError> {
        let cli = GithubCli::parse();
        req.overwrite(&cli.common);
        let req = req.validate(&repo).handle_system_error();

        let id = get_id(&cli);
        let body = get_body(&cli)?;

        let draft = cli.draft;
        let maintainer_can_modify = cli.fixup;
        let head_repo = None;

        let (our, _) = repo_parts(&repo, &req.to.remote)?;
        let (owner, owner_repo) = repo_parts(&repo, &req.from.remote)?;

        let base = req.to.branch;
        let head = format!("{}:{}", our, req.from.branch);

        let payload = CreatePullRequest {
            id,
            head,
            head_repo,
            base,
            body,
            draft,
            maintainer_can_modify,
        };

        let client = reqwest::Client::new();
        let token = find_token(&self.base).map_err(|_| GithubError::NoToken)?;
        let res =
            req::send_request(&client, &self.base, &owner, &owner_repo, &token, payload).await?;

        if cli.open {
            open::that(res.url).map_err(|_| GithubError::FailedToOpen)?;
        }

        Ok(())
    }
}

fn get_body(cli: &GithubCli) -> Result<String, GithubError> {
    let body = if let Some(desc) = &cli.description {
        match desc.read() {
            Ok(s) => s,
            Err(_) => return Err(GithubError::FailedDescription(desc.clone()).into()),
        }
    } else {
        "".to_string()
    };
    Ok(body)
}

fn get_id(cli: &GithubCli) -> req::Id {
    let id = if let Some(title) = &cli.id.title {
        req::Id::Title(title.to_string())
    } else if let Some(issue) = &cli.id.issue {
        req::Id::Issue(*issue)
    } else {
        unreachable!()
    };
    id
}

fn repo_parts(repo: &Repository, remote: &str) -> Result<(String, String), GithubError> {
    let remote = repo.find_remote(remote).unwrap();
    let url = remote.url(Direction::Push).unwrap();

    let mut path = PathBuf::from(url.path.to_string());

    let repo = path.file_name().unwrap().to_string_lossy().to_string();
    path.pop();
    let owner = path.file_name().unwrap().to_string_lossy().to_string();

    Ok((owner, repo))
}

#[async_trait::async_trait]
impl Runner for Github {
    async fn run(self, repo: Repository, req: Req) -> Result<(), DriverError> {
        self.create_pull_request(repo, req)
            .await
            .map_err(|e| DriverError::Github(e))?;

        Ok(())
    }
}
