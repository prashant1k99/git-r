use std::fs;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git-r").unwrap();
            fs::create_dir(".git-r/objects").unwrap();
            fs::create_dir(".git-r/refs").unwrap();
            fs::write(".git-r/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git-r directory")
        }
    }
}
