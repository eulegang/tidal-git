use crate::tidal::{Common, Desc};
use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct GithubCli {
    #[command(flatten)]
    pub id: Id,

    /// Description of pull request
    #[clap(short, long)]
    pub description: Option<Desc>,

    /// Open page for pull request
    #[clap(short, long)]
    pub open: bool,

    /// Draft pull request
    #[clap(short = 'D', long)]
    pub draft: bool,

    /// May maintainer ammend
    #[clap(short, long)]
    pub fixup: bool,

    #[clap(flatten)]
    pub common: Common,
}

#[derive(Args, Debug)]
#[group(required = true)]
pub struct Id {
    /// Title of the pull request
    #[clap(short, long)]
    pub title: Option<String>,

    /// Linked issue
    #[clap(short, long)]
    pub issue: Option<u64>,
}
