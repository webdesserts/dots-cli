use clap::ArgMatches;
use std::process;

use crate::dots::{self, Environment};
use crate::plan::Plan;

pub fn add(matches: &ArgMatches) {
    let env = Environment::new();
    let url = matches.value_of("REPO").expect("repo is required");
    let overwrite = matches.is_present("overwrite");
    dots::add(url, overwrite, &env)
}

pub fn install(matches: &ArgMatches) {
    let env = Environment::new();
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
    let env = Environment::new();
    let mut lines = vec![];
    for dot in dots::find_all(&env) {
        let mut remote = String::new();
        if matches.is_present("origins") {
            remote = format!(" => {}", dot.origin())
        };

        let line = format!("{name}{remote}", name = dot.package.package.name);
        lines.push(line);
    }

    print!("{}", lines.join("\n"));
}

pub fn path(matches: &ArgMatches) {
    let env = Environment::new();
    let name = matches.value_of("DOT").expect("Missing Argument <REPO>");

    match dots::find_all(&env)
        .iter()
        .find(|dot| dot.package.package.name == name)
    {
        Some(dot) => print!("{}", path = dot.path),
        None => process::exit(1),
    }
}
