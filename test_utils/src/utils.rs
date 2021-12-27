use camino::{Utf8Path, Utf8PathBuf};
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

pub fn init_git_repo<P>(path: P) -> Result<(), failure::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();

    Command::new("git")
        .arg("init")
        .current_dir(&path)
        .output()?;

    commit_all(&path, "initial commit")?;

    Ok(())
}

pub fn commit_all<P>(path: P, message: &str) -> Result<(), failure::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();

    Command::new("git")
        .arg("add")
        .arg("--all")
        .current_dir(&path)
        .output()?;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("\"{}\"", message))
        .current_dir(&path)
        .output()?;

    Ok(())
}

pub fn current_dir() -> Utf8PathBuf {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    Utf8PathBuf::from_path_buf(current_dir).unwrap()
}
