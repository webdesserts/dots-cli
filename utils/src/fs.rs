use camino::{Utf8Path, Utf8PathBuf};
use dirs::home_dir;
use std::{fs, io, process::Command};
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

        let from = Utf8Path::from_path(&entry.path()).unwrap();
        let relative = from.strip_prefix(&source).unwrap();
        let to = destination.join(&relative);

        if file_type.is_file() {
            // We're explicitely avoiding using `fs::copy` here as it sends `cargo watch` into an infinite loop
            fs::read(&from)
                .and_then(|file| fs::write(&to, &file))
                .expect(format!("Failed to copy file from {} to {}", &from, &to).as_str());
        }
        if file_type.is_dir() {
            fs::create_dir_all(&to)
                .expect(format!("Failed to copy dir from {} to {}", &from, &to).as_str());
        }
    }
    Ok(())
}

pub fn print_tree<P>(path: P) -> Result<(), failure::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();
    Command::new("tree").arg(&path).spawn()?;
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

pub fn canonicalize<P>(path: P) -> Result<Utf8PathBuf, io::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();
    path.canonicalize()
        .map(|path| Utf8PathBuf::from_path_buf(path).unwrap())
}
