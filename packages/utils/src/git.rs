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
                let err = String::from_utf8_lossy(&output.stderr);
                let out = String::from_utf8_lossy(&output.stdout);

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

pub fn get_status(dir: &Utf8Path) -> Result<String, GitError> {
    let output = map_result(
        Command::new("git")
            .arg("status")
            .arg("--porcelain=v1")
            .current_dir(dir)
            .output(),
    )?;

    let string = String::from_utf8(output.stdout).expect("unable to convert status output to utf8");

    Ok(string.trim_end().to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::ExitStatus;

    #[test]
    fn test_git_error_display_with_invalid_utf8_does_not_panic() {
        // Construct an Output with non-UTF-8 bytes (0xFF) in stderr
        // and stdout to test that from_utf8_lossy handles it gracefully
        let invalid_utf8_stderr = vec![0xFF, 0xFE, 0x80, 0x81];
        let invalid_utf8_stdout = vec![0xDE, 0xAD, 0xBE, 0xEF];

        let output = Output {
            status: ExitStatus::default(), // dummy exit status
            stdout: invalid_utf8_stdout,
            stderr: invalid_utf8_stderr,
        };

        let git_error = GitError {
            kind: GitErrorKind::Command(output),
        };

        // This should NOT panic thanks to from_utf8_lossy
        let display = format!("{}", git_error);

        // Verify the display string contains replacement characters
        assert!(display.contains("Git exited with the following output"));
    }
}
