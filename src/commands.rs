use clap::ArgMatches;
use std::fs;
use std::path::Path;
use std::process;

use git_utils;
use dots::{Dot, self};

pub fn install(matches: &ArgMatches) {

    let url = matches.value_of("REPO").expect("Missing Argument <REPO>");
    info!("adding {}", url);

    let tmp = dots::path(".tmp");

    info!("cleaning {:?}", tmp);
    clean(&tmp);

    info!("cloning...");
    git_utils::clone(url, &tmp);

    let dot = match Dot::new(&tmp) {
        Ok(dot) => dot,
        Err(_) => {
            clean(&tmp);
            error!("Repo does not appear to be a Dot");
            process::exit(1);
        }
    };

    info!("found package name: {}", dot.package.name);

    let target_dir = dots::path(dot.package.name);

    if target_dir.exists() {
        info!("path already exists {:?}", target_dir);

        if tmp.is_dir() {
            warn!("Cleaning left over .tmp directory.\nIt appears another command failed to clean up after itself.");
            clean(&target_dir);
        }
    }

    fs::rename(tmp, target_dir).expect("Error renaming repo!");
}

pub fn remove() { println!("remove has not yet been implemented!") }
pub fn update() { println!("update has not yet been implemented!") }

pub fn list(matches: &ArgMatches) {

    for dot in dots::find_all() {
        let mut remote = String::new();
        if matches.is_present("origins") {
            remote = dot.origin().map_or(remote, |origin| format!(" => {}", origin.trim()));
        };

        println!("{}{}", dot.package.name, remote)
    }
}

pub fn doctor() { println!("doctor has not yet been implemented!") }

fn clean(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}
