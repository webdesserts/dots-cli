use camino::Utf8Path;
use std::io;
use std::process::Command;

pub fn init_git_repo<P>(path: P) -> Result<(), io::Error>
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

pub fn commit_all<P>(path: P, message: &str) -> Result<(), io::Error>
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

pub fn clone<P>(url: &str, dest: P) -> Result<(), io::Error>
where
    P: AsRef<Utf8Path>,
{
    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest.as_ref())
        .arg("--depth=1")
        .output()?;

    Ok(())
}

pub fn get_origin() -> Result<Option<String>, io::Error> {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()?;

    let string = if output.status.success() {
        Some(String::from_utf8(output.stdout).expect("unable to convert origin output to utf8"))
    } else {
        None
    };

    Ok(string)
}
