use gix::{
    config::{File, Snapshot},
    Remote, Repository,
};

use super::Common;

#[derive(Debug)]
pub enum Error {
    Xyz,
    InvalidRemote,
}

pub struct Req {
    pub from: Ref,
    pub to: Ref,
}

pub struct Ref {
    pub branch: String,
    pub remote: String,
}

impl Req {
    pub fn build(repo: &Repository, opts: &Common) -> Result<Req, Error> {
        let snapshot = repo.config_snapshot();
        Ok(Req {
            from: build_from(repo, &snapshot, opts)?,
            to: build_to(repo, &snapshot, opts)?,
        })
    }
}

fn build_to(repo: &Repository, snapshot: &Snapshot, opts: &Common) -> Result<Ref, Error> {
    let remote = dbg!(build_to_remote(repo, snapshot, opts))?;

    let git_remote = repo
        .find_remote(remote.as_str())
        .map_err(|_| Error::InvalidRemote)?;

    let branch = dbg!(build_to_branch(repo, &git_remote, snapshot, opts))?;

    Ok(Ref { branch, remote })
}

fn build_from(repo: &Repository, snapshot: &Snapshot, opts: &Common) -> Result<Ref, Error> {
    let remote = dbg!(build_from_remote(repo, snapshot, opts))?;
    let branch = dbg!(build_from_branch(repo, snapshot, opts))?;

    Ok(Ref { branch, remote })
}

fn build_to_branch(
    repo: &Repository,
    remote: &Remote,
    snapshot: &Snapshot,
    opts: &Common,
) -> Result<String, Error> {
    if let Some(b) = &opts.to_branch {
        return Ok(b.to_string());
    }

    if let Ok(e) = std::env::var("TIDAL_TO_BRANCH") {
        return Ok(e);
    }

    let file: &File = snapshot;
    if let Some(x) = file.string("tidal", None, "to-branch") {
        return Ok(x.to_string());
    }

    for r in remote.refspecs(gix::remote::Direction::Push) {
        dbg!(r);
    }

    Ok(gix::init::DEFAULT_BRANCH_NAME.to_string())
}

fn build_to_remote(repo: &Repository, snapshot: &Snapshot, opts: &Common) -> Result<String, Error> {
    if let Some(r) = &opts.to_remote {
        return Ok(r.to_string());
    }

    if let Ok(e) = std::env::var("TIDAL_TO_REMOTE") {
        return Ok(e);
    }

    let file: &File = snapshot;
    if let Some(x) = file.string("tidal", None, "to-remote") {
        return Ok(x.to_string());
    }

    Ok("origin".to_string())
}

fn build_from_branch(
    repo: &Repository,
    snapshot: &Snapshot,
    opts: &Common,
) -> Result<String, Error> {
    if let Some(b) = &opts.from_branch {
        return Ok(b.to_string());
    }

    if let Ok(e) = std::env::var("TIDAL_TO_BRANCH") {
        return Ok(e);
    }

    let file: &File = snapshot;
    if let Some(x) = file.string("tidal", None, "to-branch") {
        return Ok(x.to_string());
    }

    if let Ok(Some(s)) = repo.head_name() {
        return Ok(s.shorten().to_string());
    }

    todo!()
}

fn build_from_remote(
    repo: &Repository,
    snapshot: &Snapshot,
    opts: &Common,
) -> Result<String, Error> {
    if let Some(r) = &opts.from_remote {
        return Ok(r.to_string());
    }

    if let Ok(e) = std::env::var("TIDAL_TO_REMOTE") {
        return Ok(e);
    }

    let file: &File = snapshot;
    if let Some(x) = file.string("tidal", None, "from-remote") {
        return Ok(x.to_string());
    }

    Ok("origin".to_string())
}
