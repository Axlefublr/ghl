use crate::git::Connector;
use args::Args;
use clap::Parser;
use git::open_in_browser;
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
            format!("/{0}/{1}", connector, git::rev_parse(branch)?)
        } else {
            format!("/{0}/{branch}", connector)
        }
    } else if args.path.is_some() {
        format!("/{0}/{1}", connector, git::branch()?)
    } else {
        Default::default()
    };
    let path = if let Some(path) = args.path {
        format!("/{}", git::normalize_path(path)?.display())
    } else {
        Default::default()
    };
    let link = format!("{link}{branch}{path}");
    if args.trim {
        print!("{link}")
    } else {
        println!("{link}");
    }
    if args.web {
        open_in_browser(&link)?;
    }
    Ok(())
}
