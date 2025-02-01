use anyhow::{bail, ensure, Context, Result};
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs,
    io::{self, BufRead, BufReader, Read},
};

enum Kind {
    Blob,
}

struct RateLimitedReader<R> {
    inner: R,
    remaining: usize,
}

impl<R: Read> RateLimitedReader<R> {
    fn new(inner: R, limit: usize) -> Self {
        Self {
            inner,
            remaining: limit,
        }
    }
}

impl<R: Read> Read for RateLimitedReader<R> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() > self.remaining {
            buf = &mut buf[..self.remaining + 1];
        }
        let bytes_read = self.inner.read(buf)?;
        if bytes_read > self.remaining {
            return Err(io::Error::new(
                io::ErrorKind::FileTooLarge,
                "too many bytes",
            ));
        }
        self.remaining -= bytes_read;
        Ok(bytes_read)
    }
}
pub fn cat_file(object_hash: String) -> Result<()> {
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

    let header = CStr::from_bytes_with_nul(&buf).expect("there should be only 1 null at the end");

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

    let z = RateLimitedReader::new(z, size);
    let mut z = BufReader::new(z);
    match kind {
        Kind::Blob => {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let n = io::copy(&mut z, &mut stdout).context("write .git/objects file into stdout")?;

            ensure!(
                n == size as u64,
                ".git/objects file was not the expected size (expected: {size}, received: {n})"
            );
        }
    }

    Ok(())
}
