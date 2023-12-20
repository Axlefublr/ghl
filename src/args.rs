use crate::git::Connector;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Make a link to a specific branch / commit.
    /// Branch is not specified by default, which results in making the link follow the default branch.
    #[arg(short, long)]
    pub branch: Option<String>,
    /// Parse `branch` argument with `git rev-parse`.
    /// This allows you to use things like HEAD (or just head), HEAD^, commit~2; short commit hashes become long commit hashes, branch names become their latest commit's hash.
    /// Essentially, everything that happens when you use `git rev-parse`.
    /// This is not the default because you couldn't specify links to branches that way, and would only be able to specify links to a branch's latest commit.
    #[arg(short, long)]
    pub parse: bool,
    /// Specify remote name explicitly.
    /// By default, takes the first one from `git remote`.
    /// If your repo is a fork, this will result in a link to *your* repository, rather than the upstream one.
    /// You can specify `upstream` (usually) to mean the upstream repository.
    #[arg(short, long)]
    pub remote: Option<String>,
    /// Trim the final newline of the output link
    #[arg(short, long)]
    pub trim: bool,
    /// Open the resulting link in your $BROWSER
    #[arg(short, long)]
    pub web: bool,
    /// Assume the provided filepath is literal and is relative to the repo root, and provide the connector type yourself.
    /// `tree` if you're pointing to a directory, `blob` if not.
    /// Useful when you're trying to point to a symlink, rather than the file it points to, or if you're pointing to a file you know is going to be in the remote but not locally.
    /// This is required when you're specifying a file that no longer exists, but did in some commit.
    #[arg(short, long, value_name = "WORD")]
    pub connector: Option<Connector>,
    pub path: Option<PathBuf>,
}
