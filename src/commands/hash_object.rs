use anyhow::Context;
use flate2::{write::ZlibEncoder, Compression};
use hex;
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

struct HashWriter<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

pub(crate) fn invoke(file: PathBuf, write: bool) -> anyhow::Result<()> {
    fn write_blob<W>(file: &Path, writer: W) -> anyhow::Result<String>
    where
        W: Write,
    {
        // Get file information
        let stat = fs::metadata(&file).with_context(|| format!("stat {}", file.display()))?;

        // Create the writer for zlib encodcer
        let writer = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter {
            writer,
            hasher: Sha1::new(),
        };

        // Write header content
        write!(writer, "blob ")?;
        write!(writer, "{}\0", stat.len())?;

        // Zlib compress the content and write it in the writer
        let mut file = fs::File::open(&file).with_context(|| format!("open {}", file.display()))?;
        io::copy(&mut file, &mut writer).context("stream file into blob")?;

        // finish the writer and close it
        let _ = writer.writer.finish()?;
        let hash = writer.hasher.finalize();

        Ok(hex::encode(hash))
    }

    let hash = if write {
        // We create a temporary file
        let tmp = "temporary";
        // Store the compressed data in temporary file
        let hash = write_blob(
            &file,
            std::fs::File::create(tmp).context("construct temporary file for blob")?,
        )
        .context("write out blob object")?;

        // Create a folder for the file
        fs::create_dir_all(format!(".git/objects/{}/", &hash[..2]))
            .context("create subdir of .git/objects")?;

        // Rename the temporary file to the respective file and move it in respective folder
        fs::rename(tmp, format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .context("move blob file into .git/objects")?;

        hash
    } else {
        write_blob(&file, io::sink()).context("write out blob object")?
    };

    println!("{hash}");

    Ok(())
}
