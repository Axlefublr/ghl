use std::error::Error;
use args::Args;
use clap::Parser;
use crate::git::Connector;

mod args;
mod git;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let remote = match args.remote {
        Some(remote) => remote,
        None => git::remote()?
    };
    let link = git::remote_url(&remote)?;
    let branch = if let Some(branch) = args.branch {
        if args.parse {
            format!("/{0}/{1}", Connector::Tree, git::rev_parse(&branch)?)
        } else {
            format!("/{0}/{branch}", Connector::Tree)
        }
    } else {
        String::default()
    };
    println!("{link}{branch}");
    Ok(())
}
