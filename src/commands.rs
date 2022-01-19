use clap::ArgMatches;
use std::process;

use crate::dots::{self, Environment};
use crate::plan::Plan;

pub fn add(matches: &ArgMatches) {
    let dots_path = matches.value_of("dots_path");
    let env = Environment::new(dots_path);
    let url = matches.value_of("REPO").expect("repo is required");
    let overwrite = matches.is_present("overwrite");
    dots::add(url, overwrite, &env)
}

pub fn install(matches: &ArgMatches) {
    let dots_path = matches.value_of("dots_path");
    let env = Environment::new(dots_path);
    if let Some(url) = matches.value_of("REPO") {
        let overwrite = matches.is_present("overwrite");
        dots::add(url, overwrite, &env);
    };

    let plan = match Plan::new(dots::find_all(&env), matches.is_present("force")) {
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
    let dots_path = matches.value_of("dots_path");
    let env = Environment::new(dots_path);
    for dot in dots::find_all(&env) {
        let mut remote = String::new();
        if matches.is_present("origins") {
            remote = dot
                .origin()
                .map_or(remote, |origin| format!(" => {}", origin.trim()));
        };

        println!("{name}{remote}", name = dot.package.package.name)
    }
}

pub fn prefix(matches: &ArgMatches) {
    let dots_path = matches.value_of("dots_path");
    let env = Environment::new(dots_path);
    let name = matches.value_of("DOT").expect("Missing Argument <REPO>");

    match dots::find_all(&env)
        .iter()
        .find(|dot| dot.package.package.name == name)
    {
        Some(dot) => println!("{}", path = dot.path),
        None => process::exit(1),
    }
}
