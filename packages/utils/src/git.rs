use camino::Utf8Path;
use std::error::Error;
use std::fmt;
use std::io;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct GitError {
    kind: GitErrorKind,
}

#[derive(Debug)]
pub enum GitErrorKind {
    GitNotFound,
    Command(Output),
    Io(io::Error),
}

impl Error for GitError {}

impl GitError {
    pub fn kind(&self) -> &GitErrorKind {
        &self.kind
    }
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            GitErrorKind::Io(err) => {
                write!(f, "Git failed with the following Io error:\n{}", err)
            }
            GitErrorKind::Command(output) => {
                let err = std::str::from_utf8(&output.stderr).unwrap();
                let out = std::str::from_utf8(&output.stdout).unwrap();

                write!(f, "Git exited with the following output\n{err}{out}",)
            }
            GitErrorKind::GitNotFound => {
                write!(f, r#"Unable to find "git" command"#)
            }
        }
    }
}

pub fn init_repo<P>(path: P) -> Result<(), GitError>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();

    map_result(Command::new("git").arg("init").current_dir(path).output())?;

    Ok(())
}

pub fn config<P>(path: P, key: &str, value: &str) -> Result<(), GitError>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();

    map_result(
        Command::new("git")
            .arg("config")
            .arg(key)
            .arg(value)
            .current_dir(path)
            .output(),
    )?;

    Ok(())
}

pub fn commit_all<P>(path: P, message: &str) -> Result<(), GitError>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();

    map_result(
        Command::new("git")
            .arg("add")
            .arg("--all")
            .current_dir(path)
            .output(),
    )?;

    map_result(
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(format!("\"{message}\""))
            .current_dir(path)
            .output(),
    )?;

    Ok(())
}

pub fn clone<P>(url: &str, dest: P) -> Result<(), GitError>
where
    P: AsRef<Utf8Path>,
{
    map_result(
        Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(dest.as_ref())
            .arg("--depth=1")
            .output(),
    )?;

    Ok(())
}

pub fn get_origin() -> Result<String, GitError> {
    let output = map_result(
        Command::new("git")
            .arg("remote")
            .arg("get-url")
            .arg("origin")
            .output(),
    )?;

    let string = String::from_utf8(output.stdout).expect("unable to convert origin output to utf8");

    Ok(string.trim().to_string())
}

fn map_result(result: Result<Output, io::Error>) -> Result<Output, GitError> {
    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(output)
            } else {
                Err(GitError {
                    kind: GitErrorKind::Command(output),
                })
            }
        }
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => Err(GitError {
                kind: GitErrorKind::GitNotFound,
            }),
            _ => Err(GitError {
                kind: GitErrorKind::Io(err),
            }),
        },
    }
}
