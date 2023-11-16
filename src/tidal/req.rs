use gix::{config::File, Remote, Repository};

use crate::errors::Error;

use super::Common;

#[derive(Debug)]
pub enum ReqError {
    InvalidRemote,
    InvalidBranch,
    SameRef,
    GitError(gix::reference::find::existing::Error),
}

impl std::fmt::Display for ReqError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReqError::InvalidRemote => write!(f, "invalid remote reference")?,
            ReqError::InvalidBranch => write!(f, "invalid branch reference")?,
            ReqError::SameRef => write!(f, "can't pull request the same branch of the same repository")?,
            ReqError::GitError(err) => write!(f, "invalid git reference: {err}")?,
        }

        Ok(())
    }
}

impl From<gix::reference::find::existing::Error> for ReqError {
    fn from(value: gix::reference::find::existing::Error) -> Self {
        ReqError::GitError(value)
    }
}

impl std::error::Error for ReqError {}

impl Error for ReqError {
    fn status(&self) -> i32 {
        1
    }
}

pub struct Req {
    pub from: Ref,
    pub to: Ref,
}

impl Req {
    pub fn overwrite(&mut self, opts: &Common) {
        if let Some(b) = &opts.from_branch {
            self.from.branch = b.to_string();
        }

        if let Some(r) = &opts.from_remote {
            self.from.remote = r.to_string();
        }

        if let Some(b) = &opts.to_branch {
            self.to.branch = b.to_string();
        }

        if let Some(r) = &opts.to_remote {
            self.to.remote = r.to_string();
        }
    }

    pub fn validate(self, repo: &Repository) -> Result<Self, ReqError> {
        let mut from_branch = false;
        let mut to_branch = false;
        let mut from_remote = false;
        let mut to_remote = false;

        for reference in repo.references().unwrap().local_branches().unwrap() {
            let xyz = reference.unwrap();
            let b = xyz.name().shorten();

            if b == self.from.branch {
                from_branch = true
            }

            if b == self.to.branch {
                to_branch = true
            }
        }

        for r in repo.remote_names() {
            if r == self.from.remote {
                from_remote = true
            }

            if r == self.to.remote {
                to_remote = true
            }
        }

        if !from_branch || !to_branch {
            return Err(ReqError::InvalidBranch);
        }

        if !from_remote || !to_remote {
            return Err(ReqError::InvalidRemote);
        }

        Ok(self)
    }
}

pub struct Ref {
    pub branch: String,
    pub remote: String,
}

impl Req {
    pub fn build(repo: &Repository) -> Result<Req, ReqError> {
        let snapshot = repo.config_snapshot();

        Ok(Req {
            from: build_from(repo, &snapshot)?,
            to: build_to(repo, &snapshot)?,
        })
    }
}

fn build_to(repo: &Repository, file: &File) -> Result<Ref, ReqError> {
    let remote = build_to_remote(file)?;

    let git_remote = repo
        .find_remote(remote.as_str())
        .map_err(|_| ReqError::InvalidRemote)?;

    let branch = build_to_branch(&git_remote, file)?;

    Ok(Ref { branch, remote })
}

fn build_from(repo: &Repository, file: &File) -> Result<Ref, ReqError> {
    let remote = build_from_remote(file)?;
    let branch = build_from_branch(repo, file)?;

    Ok(Ref { branch, remote })
}

fn build_to_branch(remote: &Remote, file: &File) -> Result<String, ReqError> {
    if let Ok(e) = std::env::var("TIDAL_TO_BRANCH") {
        return Ok(e);
    }

    if let Some(x) = file.string("tidal", None, "to-branch") {
        return Ok(x.to_string());
    }

    Ok(gix::init::DEFAULT_BRANCH_NAME.to_string())
}

fn build_to_remote(snapshot: &File) -> Result<String, ReqError> {
    if let Ok(e) = std::env::var("TIDAL_TO_REMOTE") {
        return Ok(e);
    }

    let file: &File = snapshot;
    if let Some(x) = file.string("tidal", None, "to-remote") {
        return Ok(x.to_string());
    }

    Ok("origin".to_string())
}

fn build_from_branch(repo: &Repository, file: &File) -> Result<String, ReqError> {
    if let Ok(e) = std::env::var("TIDAL_TO_BRANCH") {
        return Ok(e);
    }

    if let Some(x) = file.string("tidal", None, "to-branch") {
        return Ok(x.to_string());
    }

    if let Ok(Some(s)) = repo.head_name() {
        return Ok(s.shorten().to_string());
    }

    todo!()
}

fn build_from_remote(file: &File) -> Result<String, ReqError> {
    if let Ok(e) = std::env::var("TIDAL_TO_REMOTE") {
        return Ok(e);
    }

    if let Some(x) = file.string("tidal", None, "from-remote") {
        return Ok(x.to_string());
    }

    Ok("origin".to_string())
}
