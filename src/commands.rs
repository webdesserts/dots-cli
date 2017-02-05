use clap::ArgMatches;
use xdg;
use std::fs;
use std::path::Path;

use git_utils;
use dot_package::DotPackage;

pub fn add(matches: &ArgMatches) {

    let url = matches.value_of("REPO").expect("Missing Argument <REPO>");
    info!("adding {}", url);
    let xdg_dirs = xdg::BaseDirectories::new().expect("XDG Initialization Error");
    let dots = xdg_dirs.create_config_directory("dots").expect("Error creating config directory");

    let mut tmp = dots.clone();
    tmp.push(".tmp");

    info!("cleaning {}", tmp.as_path().to_str().unwrap());
    clean(tmp.as_path());

    info!("cloning...");
    git_utils::clone(url, tmp.as_path());

    let mut dot_json =  tmp.clone();
    dot_json.push("Dot.json");

    info!("parsing {:?}", dot_json);
    let dot_file = DotPackage::new(dot_json.as_path());

    info!("found package name: {}", dot_file.name);
    let mut renamed_package = tmp.clone();
    renamed_package.set_file_name(dot_file.name);

    if renamed_package.exists() {
        info!("path already exists {}", renamed_package.to_str().unwrap());

        //TODO: remove later;
        warn!("Automatically removing the previously installed package");
        clean(renamed_package.as_path());
    } else {

    }

    fs::rename(tmp.as_path(), renamed_package).expect("Error renaming repo!");
}

pub fn remove(matches: &ArgMatches) { println!("remove has not yet been implemented!") }
pub fn update(matches: &ArgMatches) { println!("update has not yet been implemented!") }

pub fn list() {

}

pub fn doctor() { println!("doctor has not yet been implemented!") }

fn clean(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).unwrap()
    }
}
