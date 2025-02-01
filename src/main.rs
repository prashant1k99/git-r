use clap::{Parser, Subcommand};
use std::fs;

mod cat_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile { object_hash: String },
    HashObject { file_name: String },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".git-r").unwrap();
            fs::create_dir(".git-r/objects").unwrap();
            fs::create_dir(".git-r/refs").unwrap();
            fs::write(".git-r/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git-r directory");
            Ok(())
        }
        Command::CatFile { object_hash } => cat_file::cat_file(object_hash),
        Command::HashObject { file_name } => Ok(()),
    }
}
