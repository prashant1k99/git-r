use std::fs;

pub(crate) fn invoke() -> anyhow::Result<()> {
    fs::create_dir(".git-r").unwrap();
    fs::create_dir(".git-r/objects").unwrap();
    fs::create_dir(".git-r/refs").unwrap();
    fs::write(".git-r/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git-r directory");
    Ok(())
}
