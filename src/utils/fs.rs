use camino::{Utf8Path, Utf8PathBuf};
use dirs::home_dir;
use std::fs;
use std::{io, process};

pub fn clean(path: &Utf8Path) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}

pub fn home() -> Utf8PathBuf {
    let home = home_dir().and_then(|p| Utf8PathBuf::from_path_buf(p).ok());

    match home {
        Some(path) => path,
        None => {
            error!("Unable to access home directory");
            process::exit(1)
        }
    }
}

pub fn canonicalize<P>(path: P) -> Result<Utf8PathBuf, io::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();
    path
        .canonicalize()
        .map(|path| Utf8PathBuf::from_path_buf(path).unwrap())
}
