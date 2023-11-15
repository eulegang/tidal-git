use gix::{bstr::BString, Repository};
use std::collections::HashSet;

use super::{github::Github, Driver};

pub fn find_driver(repo: &Repository) -> Result<Driver, DriverError> {
    let host = discover_host(repo)?;
    detect_driver(repo, &host)
}

fn detect_driver(repo: &Repository, host: &str) -> Result<Driver, DriverError> {
    let snapshot = repo.config_snapshot();
    if let Ok(section) = snapshot.section("tidal", Some(host.into())) {
        if let Some(driver) = section.value("driver") {
            let host_override = section.value("host");

            return build_driver(&driver, host, host_override.as_deref().map(Into::into));
        }
    }

    if let Some(driver) = build_default_deriver(host) {
        return Ok(driver);
    }

    Err(DriverError::NoDriverFound)
}

fn build_driver(
    ty: &[u8],
    host: &str,
    host_override: Option<&[u8]>,
) -> Result<Driver, DriverError> {
    let host = host_override.unwrap_or(host.as_bytes());
    let Ok(host) = std::str::from_utf8(host) else {
        return Err(DriverError::MalformedHost(host.to_vec()));
    };

    match ty {
        b"github" => Ok(Driver::Github(Github {
            base: host.to_string(),
        })),

        _ => Err(DriverError::UnknownDriver(ty.into())),
    }
}

fn build_default_deriver(host: &str) -> Option<Driver> {
    match host {
        "github.com" => Some(Driver::Github(Github {
            base: "api.github.com".to_string(),
        })),

        _ => None,
    }
}

fn discover_host(repo: &Repository) -> Result<String, DriverError> {
    let names = repo.remote_names();

    if names.is_empty() {
        return Err(DriverError::NoRemotes);
    }

    if let Ok(pinned) = std::env::var("TIDAL_HOST") {
        for name in names {
            let Ok(remote) = repo.find_remote(name) else {
                continue;
            };

            let Some(url) = remote.url(gix::remote::Direction::Push) else {
                continue;
            };

            let Some(host) = url.host() else {
                continue;
            };

            if host == pinned {
                return Ok(pinned);
            }
        }

        Err(DriverError::InvalidPin)
    } else {
        let mut set = HashSet::new();

        for name in names {
            let Ok(remote) = repo.find_remote(name) else {
                continue;
            };

            let Some(url) = remote.url(gix::remote::Direction::Push) else {
                continue;
            };

            let Some(host) = url.host() else { continue };

            set.insert(host.to_string());
        }

        if set.len() != 1 {
            return Err(DriverError::DivergingRemotes);
        }

        if let Some(s) = set.into_iter().next() {
            Ok(s)
        } else {
            Err(DriverError::NoRemotes)
        }
    }
}

#[derive(Debug)]
pub enum DriverError {
    NoRemotes,
    InvalidPin,
    DivergingRemotes,
    NoDriverFound,
    UnknownDriver(BString),
    MalformedHost(Vec<u8>),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::NoRemotes => write!(f, "no remotes are configured")?,
            DriverError::InvalidPin => write!(f, "pinned host is not in remotes")?,
            DriverError::DivergingRemotes => write!(f, "all remotes do not have the same host")?,
            DriverError::NoDriverFound => write!(f, "failed to find driver")?,
            DriverError::UnknownDriver(name) => {
                if let Ok(name) = std::str::from_utf8(name) {
                    write!(f, "unknown driver: {name}")?;
                } else {
                    write!(f, "unknown driver: {name}")?;
                }
            }

            DriverError::MalformedHost(host) => {
                write!(f, "malformed host {}", String::from_utf8_lossy(host))?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for DriverError {}
impl crate::errors::Error for DriverError {
    fn status(&self) -> i32 {
        5
    }
}
