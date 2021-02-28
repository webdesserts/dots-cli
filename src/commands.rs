use clap::ArgMatches;
use std::process;

use dots;
use plan::Plan;

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

    let plan = match Plan::new(dots::find_all(), matches.is_present("force")) {
        Ok(plan) => {
            info!("Looks Good! Nothing wrong with the current install plan!");
            plan
        }
        Err(err) => {
            println!();
            error!("{}", err);
            error!("Currently defined install would fail!");
            process::exit(1)
        }
    };

    if matches.is_present("dry") {
        process::exit(1)
    } else {
        match plan.execute(matches.is_present("force")) {
            Ok(_) => {
                info!("Install was a success!");
                process::exit(0)
            }
            Err(err) => {
                error!("Install Failed!");
                error!("{}", err);

                process::exit(1)
            }
        }
    }
}

pub fn list(matches: &ArgMatches) {
    for dot in dots::find_all() {
        let mut remote = String::new();
        if matches.is_present("origins") {
            remote = dot
                .origin()
                .map_or(remote, |origin| format!(" => {}", origin.trim()));
        };

        println!("{}{}", dot.package.package.name, remote)
    }
}

pub fn prefix(matches: &ArgMatches) {
    let name = matches.value_of("DOT").expect("Missing Argument <REPO>");

    match dots::find_all()
        .iter()
        .find(|dot| dot.package.package.name == name)
    {
        Some(dot) => println!("{}", dot.path),
        None => process::exit(1),
    }
}
