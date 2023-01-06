use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use dirs::home_dir;
use std::{fs, io, os};
use walkdir::WalkDir;

pub fn copy_dir<S, D>(source: S, destination: D) -> Result<(), io::Error>
where
    S: AsRef<Utf8Path>,
    D: AsRef<Utf8Path>,
{
    let source = source.as_ref();
    let destination = destination.as_ref();

    for entry in WalkDir::new(source).max_depth(10) {
        let entry = entry?;
        let file_type = entry.file_type();

        let from = Utf8Path::from_path(entry.path()).unwrap();
        let relative = from.strip_prefix(source).unwrap();
        let to = destination.join(relative);

        if file_type.is_file() {
            // We're explicitely avoiding using `fs::copy` here as it sends `cargo watch` into an infinite loop
            fs::read(from)
                .and_then(|file| fs::write(&to, file))
                .unwrap_or_else(|_| panic!("Failed to copy file from {from} to {to}"));
        }
        if file_type.is_dir() {
            fs::create_dir_all(&to)
                .unwrap_or_else(|_| panic!("Failed to copy dir from {from} to {to}"));
        }
    }
    Ok(())
}

pub fn current_dir() -> Utf8PathBuf {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    Utf8PathBuf::from_path_buf(current_dir).expect("Unable to parse current directory as utf8")
}

pub fn home() -> Utf8PathBuf {
    let home = home_dir().expect("unable to get home directory");
    Utf8PathBuf::from_path_buf(home).expect("Unable to parse home directory as utf8")
}

pub fn clean(path: &Utf8Path) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}

pub fn empty_git_directory(path: &Utf8Path) -> anyhow::Result<()> {
    let read_dir = fs::read_dir(path)?;

    for entry in read_dir {
        let entry = entry?;
        let path = Utf8PathBuf::try_from(entry.path())?;
        if path.file_name() == Some(".git") {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else if path.is_file() {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

pub fn soft_link<P>(link_path: P, file_path: P) -> Result<(), io::Error>
where
    P: AsRef<Utf8Path>,
{
    let link_path = link_path.as_ref();
    let file_path = file_path.as_ref();
    os::unix::fs::symlink(file_path, link_path)?;
    Ok(())
}

pub fn canonicalize<P>(path: P) -> Result<Utf8PathBuf, io::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();
    path.canonicalize()
        .map(|path| Utf8PathBuf::from_path_buf(path).unwrap())
}
