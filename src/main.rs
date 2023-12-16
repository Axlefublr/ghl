use args::Args;
use clap::Parser;

mod args;

fn main() {
    let Args { commit, file } = Args::parse();
}
