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
        .arg("--shallow-submodules")
        .spawn()
        .map_err(|err| {
            match err.kind() {
                io::ErrorKind::NotFound => {
                    error!(r#"Unable to find "git" command"#);
                    process::exit(1)
                },
                _ => err
            }
        }).expect("Failed to execute Command");

    let status = child.wait().expect("Failed to wait on git clone");
    println!();

    if !status.success() {
        error!("Could not clone {}", url);
        process::exit(1);
    }
}
