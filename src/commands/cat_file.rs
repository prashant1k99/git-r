use crate::objects::{Kind, Object};
use std::io;

use anyhow::{Context, Result};

pub(crate) fn invoke(object_hash: &str, pretty_print: bool) -> Result<()> {
    anyhow::ensure!(
        pretty_print,
        "please provide the '-p' flag for pretty print"
    );

    let mut object = Object::read(object_hash).context("parse out blob object file")?;
    match object.kind {
        Kind::Blob => {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let n = io::copy(&mut object.reader, &mut stdout)
                .context("write .git/objects file to stdout")?;
            anyhow::ensure!(
                n == object.expected_size,
                ".git/object file was not the expected size (expected: {}, actual: {n})",
                object.expected_size
            );
        }
        Kind::Tree => super::ls_tree::handle_tree(false, object)?,
        _ => anyhow::bail!("implementation not completed for '{}'", object.kind),
    }
    Ok(())
}
