mod cat_file;
mod hash_object;
mod init;

use std::path::PathBuf;

use cat_file::cat_file;
use clap::{Parser, Subcommand};
use hash_object::hash_object;
use init::init;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile {
        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file_name: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => init(),
        Command::CatFile { object_hash } => cat_file(object_hash),
        Command::HashObject { file_name, write } => hash_object(file_name, write),
    }
}
