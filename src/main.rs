use anyhow::{bail, ensure, Context};
use clap::{Parser, Subcommand, ValueEnum};
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, ValueEnum)]
enum FileType {
    Blob,
    Commit,
    Tree,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
    CatFile { object_hash: String },
}

enum Kind {
    Blob,
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
        }
        Command::CatFile { object_hash } => {
            // TODO: Need to handle short version of hash as well
            let f = fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .context("Open in .git/objects")?;

            let z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);

            let mut buf = Vec::new();

            // Read until the first null byte to read the header
            z.read_until(0, &mut buf)
                .context("read header from .git/objects")?;

            let header =
                CStr::from_bytes_with_nul(&buf).expect("there should be only 1 null at the end");

            let header = header
                .to_str()
                .context(".git/objects file header isn't valid UTFD-8")?;

            let Some((kind, size)) = header.split_once(" ") else {
                bail!(".git/objects file header did not start with a known type: '{header}'")
            };

            let kind = match kind {
                "blob" => Kind::Blob,
                _ => bail!("we do not support printing of {kind}"),
            };

            let size = size
                .parse::<usize>()
                .context(".git/objects file header has invalid size: {size}")?;

            buf.clear();
            buf.reserve_exact(size);

            unsafe {
                // It is safe as we are filling the space just below
                buf.set_len(size);
            }

            z.read_exact(&mut buf[..])
                .context(".git/objects file content did not match expactations")?;

            // To confirm that nothing is remaining post this
            let n = z
                .read(&mut [0])
                .context("validate EOF in .git/objects file")?;

            ensure!(n == 0, ".git/objects file had {n} trailing bytes");

            // We need to handle all types, as it can be anything as well as UTF-8, images, etc..
            let stdout = io::stdout();
            let mut stdout = stdout.lock();

            match kind {
                Kind::Blob => {
                    stdout
                        .write_all(&buf)
                        .context("write object contents to stdout")?;
                }
            }
        }
    }

    Ok(())
}
