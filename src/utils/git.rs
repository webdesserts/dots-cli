use std::io;

use std::process::{self, Command};

use camino::Utf8Path;

pub fn clone<P>(url: &str, dest: P)
where
    P: AsRef<Utf8Path>,
{
    println!();
    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest.as_ref())
        .arg("--depth=1")
        .output()
        .map_err(require_git)
        .expect("Failed to execute Command");

    println!();

    if !output.status.success() {
        error!("Could not clone {}", url);
        process::exit(1);
    }
}

pub fn get_origin() -> Option<String> {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .map_err(require_git)
        .expect("Failed to execute Command");

    if output.status.success() {
        match String::from_utf8(output.stdout) {
            Ok(string) => Some(string),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn require_git(err: io::Error) -> io::Error {
    match err.kind() {
        io::ErrorKind::NotFound => {
            error!(r#"Unable to find "git" command"#);
            process::exit(1)
        }
        _ => err,
    }
}
