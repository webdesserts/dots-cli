#[macro_use]
extern crate clap;
extern crate xdg;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate colored;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod dots;
mod dot_package;
mod git_utils;
mod commands;

use colored::*;
use env_logger::LogBuilder;
use log::{LogRecord, LogLevelFilter};

fn main() {
    let mut builder = LogBuilder::new();

    let log_format = |record: &LogRecord| {
        use log::LogLevel::*;
        let level = match record.level() {
            Debug => "[debug]".bold(),
            Info => "[info]".blue().bold(),
            Warn => "[warn]".yellow().bold(),
            Error => "[error]".red().bold(),
            Trace => "[trace]".bold(),
        };
        format!("{} {}", level, record.args())
    };

    builder
        .format(log_format)
        .filter(None, LogLevelFilter::Info)
        .init().unwrap();

    let app = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (about: crate_description!())
        (author: crate_authors!("\n"))
        (@subcommand install =>
            (about: "Downloads and installs the given git repo as a dot")
            (@arg REPO: +required "a git url that points to a Dot repo containing all your dotfiles")
        )
        (@subcommand remove =>
            (about: "Removes a dot with the given name")
        )
        (@subcommand update =>
            (about: "Updates all dots")
        )
        (@subcommand list =>
            (@arg origins: --origins "list the git origin of each dot")
            (alias: "ls")
            (about: "List the names of all installed dots and the repos they link to")
        )
        (@subcommand doctor =>
            (about: "Checks to make sure all files and symlinks are correctly applied")
        )
    );

    let matches = app.get_matches();

    match matches.subcommand() {
        ("install", Some(sub_matches)) => commands::install(sub_matches),
        ("remove", _) => commands::remove(),
        ("update", _) => commands::update(),
        ("list", Some(sub_matches)) => commands::list(sub_matches),
        ("doctor", _) => commands::doctor(),
        _ => { println!("{}", matches.usage()) }
    }
}
