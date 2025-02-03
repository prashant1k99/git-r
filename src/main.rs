use std::path::PathBuf;

pub(crate) mod commands;
pub(crate) mod objects;

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
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file_name: PathBuf,
    },
    LsTree {
        #[arg(long)]
        name_only: bool,

        tree_sha: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => commands::init::invoke(),
        Command::CatFile {
            object_hash,
            pretty_print,
        } => commands::cat_file::invoke(&object_hash, pretty_print),
        Command::HashObject { file_name, write } => commands::hash_object::invoke(file_name, write),
        Command::LsTree {
            tree_sha,
            name_only,
        } => commands::ls_tree::invoke(&tree_sha, name_only),
    }
}
