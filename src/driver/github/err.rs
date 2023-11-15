use crate::{errors::Error, tidal::Desc};
use std::fmt::Display;

#[derive(Debug)]
pub enum GithubError {
    FailedDescription(Desc),
    Http(reqwest::Error),
    Forbidden,
    Validation,
    NoToken,
    FailedToOpen,
}

impl From<reqwest::Error> for GithubError {
    fn from(value: reqwest::Error) -> Self {
        if let Some(status) = value.status() {
            match status.into() {
                403 => return GithubError::Forbidden,
                422 => return GithubError::Validation,
                _ => (),
            }
        }

        GithubError::Http(value)
    }
}

impl std::error::Error for GithubError {}

impl Display for GithubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubError::Forbidden => write!(f, "not permitted to create pull request")?,
            GithubError::Validation => write!(f, "pull request failed validation")?,
            GithubError::FailedDescription(_) => write!(f, "failed to read description")?,
            GithubError::NoToken => write!(f, "no token found")?,
            GithubError::Http(e) => write!(f, "http error: {e}")?,
            GithubError::FailedToOpen => write!(f, "failed to open pull request in browser")?,
        }

        Ok(())
    }
}

impl Error for GithubError {
    fn status(&self) -> i32 {
        2
    }
}
