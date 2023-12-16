use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub enum Connector {
    Blob,
    Tree
}

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blob => write!(f, "blob"),
            Self::Tree => write!(f, "tree")
        }
    }
}
fn sh(command: &mut Command) -> Result<String, Box<dyn Error>> {
    let output = command.output()?;
    if !output.status.success() {
        Err(String::from_utf8(output.stderr)?.trim())?
    } else {
        Ok(String::from_utf8(output.stdout)?.trim().to_owned())
    }
}

pub fn branch() -> Result<String, Box<dyn Error>> {
    sh(Command::new("git").arg("branch").arg("--show-current"))
}

pub fn remote() -> Result<String, Box<dyn Error>> {
    Ok(sh(Command::new("git").arg("remote"))?.lines().take(1).collect())
}

pub fn remote_url(remote: &str) -> Result<String, Box<dyn Error>> {
    let link = sh(Command::new("git").arg("remote").arg("get-url").arg(remote))?;
    Ok(link.strip_suffix(".git").unwrap_or(&link).to_owned())
}

pub fn rev_parse(rev: String) -> Result<String, Box<dyn Error>> {
    let rev = if rev.starts_with("head") {
        rev.replace("head", "HEAD")
    } else {
        rev
    };
    sh(Command::new("git").arg("rev-parse").arg(rev))
}

fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(sh(Command::new("git").arg("rev-parse").arg("--show-toplevel"))?.parse()?)
}

pub fn connector(path: &Path) -> Result<Connector, Box<dyn Error>> {
    match fs::symlink_metadata(path)?.is_dir() {
        true => Ok(Connector::Tree),
        false => Ok(Connector::Blob)
    }
}