use gix::Repository;

use crate::{errors::Error, tidal::Req};

use self::github::GithubError;

pub mod detect;
mod github;

pub enum Driver {
    Github(github::Github),
}

#[derive(Debug)]
pub enum DriverError {
    Github(github::GithubError),
}

#[async_trait::async_trait]
pub trait Runner {
    async fn run(self, repo: Repository, req: Req) -> Result<(), DriverError>;
}

#[async_trait::async_trait]
impl Runner for Driver {
    async fn run(self, repo: Repository, req: Req) -> Result<(), DriverError> {
        match self {
            Driver::Github(inner) => inner.run(repo, req).await,
        }
    }
}

impl Error for DriverError {
    fn status(&self) -> i32 {
        match self {
            DriverError::Github(inner) => inner.status(),
        }
    }
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::Github(inner) => inner.fmt(f),
        }
    }
}

impl std::error::Error for DriverError {}

impl From<GithubError> for DriverError {
    fn from(value: GithubError) -> Self {
        DriverError::Github(value)
    }
}
