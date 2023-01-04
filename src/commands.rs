use std::process;

use crate::dots::{self, Environment};
use crate::fs_manager::FSManager;
use crate::plan::Plan;

pub fn add(url: &str, overwrite: bool) {
    let env = Environment::new();
    dots::add(url, overwrite, &env)
}

pub fn install(repo: &Option<String>, overwrite: bool, force: bool, dry: bool) {
    let env = Environment::new();
    if let Some(url) = repo {
        dots::add(url, overwrite, &env);
    };
    let dots = dots::find_all(&env);

    let mut plan = Plan::new(force);

    let mut fs_manager = FSManager::init(&env);
    plan.clean(&env, &mut fs_manager, &dots)
        .unwrap_or_else(|err| {
            error!("failed to clean current install:");
            error!("{}", err);
            process::exit(1);
        });

    /* Validate whether the plan passes or fails */
    match plan.validate(dots) {
        Ok(plan) => {
            info!("Looks Good! Nothing wrong with the current install plan!");
            plan
        }
        Err(err) => {
            error!("{}", err);
            error!("Currently defined install would fail!");
            process::exit(1)
        }
    }

    if dry {
        process::exit(1)
    } else {
        match plan.execute(&mut fs_manager, force) {
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

pub fn list(origins: bool) {
    let env = Environment::new();
    let mut lines = vec![];
    for dot in dots::find_all(&env) {
        let mut remote = String::new();
        if origins {
            remote = format!(" => {}", dot.origin())
        };

        let line = format!("{name}{remote}", name = dot.package.name);
        lines.push(line);
    }

    print!("{}", lines.join("\n"));
}

pub fn path(name: &str) {
    let env = Environment::new();

    match dots::find_all(&env)
        .iter()
        .find(|dot| dot.package.name == name)
    {
        Some(dot) => print!("{path}", path = dot.path),
        None => process::exit(1),
    }
}
