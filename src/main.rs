use crate::git::Connector;
use args::Args;
use clap::Parser;
use std::error::Error;

mod args;
mod git;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let remote = match args.remote {
        Some(remote) => remote,
        None => git::remote()?,
    };
    let link = git::remote_url(&remote)?;
    let connector = if let Some(ref path) = args.path {
        git::connector(path)?
    } else {
        Connector::Tree
    };
    let branch = if let Some(branch) = args.branch {
        if args.parse {
            Some(format!("/{0}/{1}", connector, git::rev_parse(branch)?))
        } else {
            Some(format!("/{0}/{branch}", connector))
        }
    } else {
        None
    };
    let ensured_branch: String;
    let path = if let Some(path) = args.path {
        ensured_branch = match branch {
            Some(branch) => branch,
            None => format!("/{0}/{1}", connector, git::branch()?),
        };
        format!("/{}", git::normalize_path(path)?.display())
    } else {
        ensured_branch = branch.unwrap_or_default();
        Default::default()
    };
    println!("{link}{ensured_branch}{path}");
    Ok(())
}
