use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

mod req;

pub use req::{Ref, Req, ReqError};

#[derive(Parser, Debug)]
pub struct Common {
    /// The branch to merge from
    #[clap(short = 'b', long = "from-branch")]
    pub from_branch: Option<String>,
    /// The branch to merge to
    #[clap(short = 'B', long = "to-branch")]
    pub to_branch: Option<String>,
    /// The remote to merge from
    #[clap(short = 'r', long = "from-remote")]
    pub from_remote: Option<String>,
    /// The remote to merge to
    #[clap(short = 'R', long = "to-remote")]
    pub to_remote: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Desc {
    Inline(String),
    Stdin,
    File(PathBuf),
}

impl FromStr for Desc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            return Ok(Desc::Stdin);
        }

        if let Some(s) = s.strip_prefix('@') {
            return Ok(Desc::File(PathBuf::from(s)));
        }

        Ok(Desc::Inline(s.to_string()))
    }
}

impl Desc {
    pub fn read(&self) -> io::Result<String> {
        match self {
            Desc::Inline(s) => Ok(s.clone()),
            Desc::Stdin => {
                let mut str = String::new();
                io::stdin().lock().read_to_string(&mut str)?;
                Ok(str)
            }
            Desc::File(path) => fs::read_to_string(&path),
        }
    }
}
