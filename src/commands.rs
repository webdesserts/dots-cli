use clap::ArgMatches;
use std::process;

use install;
use dots;

pub fn add(matches: &ArgMatches) {
    let url = matches.value_of("REPO").expect("repo is required");
    let overwrite = matches.is_present("overwrite");
    dots::add(url, overwrite)
}

pub fn install(matches: &ArgMatches) {
    if let Some(url) = matches.value_of("REPO") {
        let overwrite = matches.is_present("overwrite");
        dots::add(url, overwrite);
    };

    match install::Plan::new(dots::find_all(), matches.is_present("force")) {
        Ok(_) => {
            info!("Looks Good! Nothing wrong with the current install plan!")
        },
        Err(err) => {
            println!();
            error!("{}", err);
            error!("Currently defined install would fail!");
            process::exit(1)
        }
    };
}

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
        None => { process::exit(1) },
    }
}
