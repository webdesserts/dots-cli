use clap::ArgMatches;
use std::process;

use dots;
use link;

pub fn add(matches: &ArgMatches) {
    let url = matches.value_of("REPO").expect("repo is required");
    let overwrite = matches.is_present("overwrite");
    dots::add(url, overwrite)
}

pub fn install(matches: &ArgMatches) {
    if let Some(url) = matches.value_of("REPO") {
        let overwrite = matches.is_present("overwrite");
        dots::add(url, overwrite)
    }

    match link::Plan::new(dots::find_all(), matches.is_present("force")) {
        Ok(plan) => plan,
        Err(err) => {
            error!("{}", err);
            error!("Please resolve errors and run install again");
            process::exit(1)
        }
    };

    /*
    match link::install(plan) {
        Ok(_) => { info!("Successfully Installed!") }
        Err(err) => {
            error!("Despite our best efforts, we still ran into an issue while installing, Sorry :(\n{}", err);
            process::exit(1)
        }
    };
    */
}

pub fn remove() { println!("remove has not yet been implemented!") }
pub fn uninstall() { println!("uninstall has not yet been implemented!") }
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

pub fn prefix(matches: &ArgMatches) {
    let name = matches.value_of("DOT").expect("Missing Argument <REPO>");

    match dots::find_all().iter().find(|dot| dot.package.name == name) {
        Some(dot) => println!("{}", dot.path.to_str().unwrap()),
        None => (),
    }
}

pub fn doctor() { println!("doctor has not yet been implemented!") }
