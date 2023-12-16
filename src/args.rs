use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Make a link to a specific branch / commit. Branch is not specified by default, which results in making the link follow the default branch.
    #[arg(short, long)]
    pub branch: Option<String>,
    /// Parse `branch` argument with `git rev-parse`. This allows you to use things like HEAD (or just head), HEAD^, commit~2; short commit hashes become long commit hashes, branch names become their latest commit's hash. Essentially, everything that happens when you use `git rev-parse`. This is not the default because you couldn't specify links to branches that way, and would only be able to specify links to a branch's latest commit.
    #[arg(short, long)]
    pub parse: bool,
    /// Remote. By default, takes the first one from `git remote`. If your repo is a fork, this will result in a link to *your* repository, rather than the upstream one. Same case if your repo isn't a fork.
    #[arg(short, long)]
    pub remote: Option<String>,
    pub file: Option<PathBuf>,
}
