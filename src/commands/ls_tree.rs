use anyhow::Context;
use std::{
    ffi::CStr,
    io::{BufRead, Write},
};

use crate::objects::{Kind, Object};

pub(crate) fn handle_tree(name_only: bool, mut object: Object<impl BufRead>) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    let mut hashbuf = [0; 20];
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    loop {
        buf.clear();
        let n = object
            .reader
            .read_until(0, &mut buf)
            .context("read tree entry object hash")?;

        if n == 0 {
            // It's the file end
            break;
        }

        object
            .reader
            .read_exact(&mut hashbuf[..])
            .context("read tree entry object hash")?;

        let mode_and_name = CStr::from_bytes_with_nul(&buf).context("invalid tree entry")?;

        let mut bits = mode_and_name.to_bytes().splitn(2, |&b| b == b' ');

        let mode = bits.next().expect("split always yields once");
        let name = bits
            .next()
            .ok_or_else(|| anyhow::anyhow!("tree entry has no file name"))?;

        if name_only {
            stdout
                .write_all(name)
                .context("write tree entry name to stdout")?;
        } else {
            let mode = std::str::from_utf8(mode).context("mode is always valid utf-8")?;
            let hash = hex::encode(&hashbuf);
            let object = Object::read(&hash)
                .with_context(|| format!("read object for tree entry {hash}"))?;
            write!(stdout, "{mode:0>6} {} {hash} ", object.kind)
                .context("write tree entry meta to stdout")?;
            stdout
                .write_all(name)
                .context("write tree entry name to stdout")?;
        }
        writeln!(stdout, "").context("write newline to stdout")?;
    }
    Ok(())
}

pub(crate) fn invoke(tree_sha: &str, name_only: bool) -> anyhow::Result<()> {
    let object = Object::read(tree_sha).context("parse out of tree failed")?;
    match object.kind {
        Kind::Tree => handle_tree(name_only, object)?,
        _ => anyhow::bail!("Do not support '{}' in ls-tree", object.kind),
    }
    Ok(())
}
