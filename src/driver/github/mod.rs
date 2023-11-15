use super::{DriverError, Runner};
use crate::cred::find_token;
use crate::driver::github::cli::GithubCli;
use crate::driver::github::req::CreatePullRequest;
use clap::Parser;

pub use err::GithubError;
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
    async fn create_pull_request(self, repo: Repository) -> Result<(), GithubError> {
        let cli = GithubCli::parse();

        let client = reqwest::Client::new();

        let body = if let Some(desc) = cli.description {
            match desc.read() {
                Ok(s) => s,
                Err(_) => return Err(GithubError::FailedDescription(desc).into()),
            }
        } else {
            "".to_string()
        };

        let token = find_token(&self.base).map_err(|_| GithubError::NoToken)?;

        let id = if let Some(title) = cli.id.title {
            req::Id::Title(title)
        } else if let Some(issue) = cli.id.issue {
            req::Id::Issue(issue)
        } else {
            unreachable!()
        };

        let draft = cli.draft;
        let maintainer_can_modify = cli.fixup;
        let head_repo = None;

        let s = crate::tidal::Req::build(&repo, &cli.common);

        let base = "".to_string();
        let head = "".to_string();
        let owner = "".to_string();
        let repo = "".to_string();

        let payload = CreatePullRequest {
            id,
            head,
            head_repo,
            base,
            body,
            draft,
            maintainer_can_modify,
        };

        dbg!(payload);

        return Ok(());

        let res = req::send_request(&client, &self.base, &owner, &repo, &token, payload).await?;

        if cli.open {
            open::that(res.url).map_err(|_| GithubError::FailedToOpen)?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Runner for Github {
    async fn run(self, repo: Repository) -> Result<(), DriverError> {
        self.create_pull_request(repo)
            .await
            .map_err(|e| DriverError::Github(e))?;

        Ok(())
    }
}
