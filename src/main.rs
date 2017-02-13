#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate colored;

#[macro_use]
extern crate serde_derive;
extern crate toml;

mod dots;
mod dot_package;
mod commands;
mod install;
mod utils;

use env_logger::LogBuilder;
use log::{LogRecord, LogLevelFilter};

fn main() {
    let mut builder = LogBuilder::new();

    let log_format = |record: &LogRecord| {
        use colored::*;
        use log::LogLevel::*;
        let level = match record.level() {
            Debug => "[debug]".bold(),
            Info => "[info]".blue().bold(),
            Warn => "[warn]".yellow().bold(),
            Error => "[error]".red().bold(),
            Trace => "[trace]".bold(),
        };
        let string = format!("{}", record.args());
        let indented = string.lines().enumerate().map(|(i, line)| {
            let indent = if i == 0 { "" } else { "  " };
            format!("{} {}{}\n", level, indent, line)
        }).collect::<String>();
        format!("{}", indented)
    };

    builder
        .format(log_format)
        .filter(None, LogLevelFilter::Info)
        .init().unwrap();

    let app = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (about: crate_description!())
        (author: crate_authors!("\n"))
        (@subcommand add =>
            (about: "Downloads the given git repo as a dot")
            (@arg REPO: +required "A git url that points to a Dot repo containing all your dotfiles")
            (@arg overwrite: --overwrite "Will remove pre-existing packages of the same name")
        )
        (@subcommand install =>
            (about: "Installs all Dots")
            (@arg REPO: "An optional git url that points to a Dot repo that you want to add before installing")
            (@arg overwrite: --overwrite "Will remove pre-existing dots of the same name")
            (@arg force: -f --force "Will remove pre-existing directories when creating symlinks")
            (@arg dry: --dry "run through the install plan without actually making any changes")
        )
        (@subcommand remove =>
            (about: "Removes a dot with the given name")
        )
        (@subcommand uninstall =>
            (about: "Removes a dot with the given name and re-installs all dots")
        )
        (@subcommand update =>
            (about: "Updates all dots")
        )
        (@subcommand list =>
            (@arg origins: --origins "list the git origin of each dot")
            (alias: "ls")
            (about: "List the names of all installed dots and the repos they link to")
        )
        (@subcommand prefix =>
            (@arg DOT: +required "The dot package name that you would like to search for")
            (about: "returns the installed location of a given dot")
        )
        /*
        (@subcommand doctor =>
            (about: "Checks to make sure that the previous install's symlinks still work")
        )
        */
    );

    let matches = app.get_matches();

    match matches.subcommand() {
        ("add", Some(sub_matches)) => commands::add(sub_matches),
        ("install", Some(sub_matches)) => commands::install(sub_matches),
        ("list", Some(sub_matches)) => commands::list(sub_matches),
        ("prefix", Some(sub_matches)) => commands::prefix(sub_matches),
        _ => { println!("{}", matches.usage()) }
    }
}
