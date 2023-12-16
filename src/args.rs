use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
	// Commit hash. First 7 chars or full. HEAD by default.
	#[arg(short, long)]
	pub commit: Option<String>,
	pub file: Option<PathBuf>
}