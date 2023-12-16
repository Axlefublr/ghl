use args::Args;
use clap::Parser;

mod args;
mod git;

fn main() {
    let args = Args::parse();
}
