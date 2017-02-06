use clap::ArgMatches;
use std::fs;
use std::path::Path;
use std::process;

use git_utils;
use dots::{Dot, self};

pub fn add(matches: &ArgMatches) {

    let url = matches.value_of("REPO").expect("Missing Argument <REPO>");
    info!("adding {}", url);

    let tmp = dots::path(".tmp");

    info!("cleaning {:?}", tmp);
    clean(&tmp);

    info!("cloning...");
    git_utils::clone(url, &tmp);

    let dot = match Dot::new(&tmp) {
        Ok(dot) => dot,
        Err(err) => {
            clean(&tmp);
            error!("Repo does not appear to be a Dot");
            process::exit(1);
        }
    };

    info!("found package name: {}", dot.package.name);

    let target_dir = dots::path(dot.package.name);

    if target_dir.exists() {
        info!("path already exists {:?}", target_dir);

        //TODO: remove later;
        warn!("Automatically removing the previously installed package");
        clean(&target_dir);
    }

    fs::rename(tmp, target_dir).expect("Error renaming repo!");
}

pub fn remove(matches: &ArgMatches) { println!("remove has not yet been implemented!") }
pub fn update(matches: &ArgMatches) { println!("update has not yet been implemented!") }

pub fn list() { }

pub fn doctor() { println!("doctor has not yet been implemented!") }

fn clean(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}
