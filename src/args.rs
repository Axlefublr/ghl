use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Make a link to a specific branch. Branch is not specified by default, which results in making the link follow the default branch.
    #[arg(short, long)]
    pub branch: Option<String>,
    /// Remote. By default, takes the first one from `git remote`. If your repo is a fork, this will result in a link to *your* repository, rather than the upstream one. Same case if your repo isn't a fork.
    #[arg(short, long)]
    pub remote: Option<String>,
    pub file: Option<PathBuf>,
}
