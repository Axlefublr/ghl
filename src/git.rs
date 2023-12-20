use clap::ValueEnum;
use std::env;
use std::env::VarError;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const PATH_NOT_FOUND_ERROR: &str = "Couldn't resolve specified path to a real file / directory. Use --connector to specify the connector type yourself.";

#[derive(Clone, Copy, ValueEnum)]
pub enum Connector {
    Blob,
    Tree,
}

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blob => write!(f, "blob"),
            Self::Tree => write!(f, "tree"),
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

pub fn connector(path: &Path, connector: Option<Connector>) -> Result<Connector, Box<dyn Error>> {
    if let Some(connector) = connector {
        return Ok(connector);
    }
    match fs::symlink_metadata(path).map_err(|_| PATH_NOT_FOUND_ERROR)?.is_dir() {
        true => Ok(Connector::Tree),
        false => Ok(Connector::Blob),
    }
}

fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(sh(Command::new("git").arg("rev-parse").arg("--show-toplevel"))?.parse()?)
}

pub fn normalize_path(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let current = path.canonicalize().map_err(|_| PATH_NOT_FOUND_ERROR)?;
    Ok(current.strip_prefix(repo_root()?)?.into())
}

pub fn open_in_browser(link: &str) -> Result<(), Box<dyn Error>> {
    let browser = match env::var("BROWSER") {
        Ok(path) => path,
        Err(error) => {
            if let VarError::NotPresent = error {
                Err("BROWSER environment variable is undefined.")?
            } else {
                Err(error)?
            }
        }
    };
    let _ = sh(Command::new(browser).arg(link)).unwrap();
    Ok(())
}
