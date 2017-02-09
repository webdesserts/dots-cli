use std::process::{Command, self};
use std::path::Path;
use std::io;

pub fn clone(url: &str, dest: &Path) {
    let dest_str = dest.to_str().unwrap();

    println!();
    let mut child = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dest_str)
        .arg("--depth=1")
        .spawn()
        .map_err(require_git)
        .expect("Failed to execute Command");

    let status = child.wait().expect("Failed to wait on git clone");
    println!();

    if !status.success() {
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
            Err(_) => None
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
        },
        _ => err
    }
}
