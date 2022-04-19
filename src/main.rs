extern crate anyhow;
extern crate camino;
extern crate tempfile;
extern crate utils;
#[macro_use]
extern crate clap;
extern crate dirs;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod commands;
mod dot_package;
pub mod dots;
pub mod plan;

use std::io::Write;

use clap::{crate_name, Command};
use env_logger::fmt::Formatter;
use env_logger::Builder;
use utils::stylize::Stylable;

mod styles {
    use utils::{style, stylize::Style};

    const LOG: Style = style! { Bold };

    pub const DEBUG_LOG: Style = LOG;
    pub const INFO_LOG: Style = LOG.blue();
    pub const WARN_LOG: Style = LOG.yellow();
    pub const ERROR_LOG: Style = LOG.red();
    pub const TRACE_LOG: Style = LOG;
}

fn main() {
    let mut builder = Builder::new();

    let log_format = |buf: &mut Formatter, record: &log::Record| -> Result<(), std::io::Error> {
        use log::Level::*;
        let level = match record.level() {
            Debug => "[debug]".apply_style(styles::DEBUG_LOG),
            Info => "[info]".apply_style(styles::INFO_LOG),
            Warn => "[warn]".apply_style(styles::WARN_LOG),
            Error => "[error]".apply_style(styles::ERROR_LOG),
            Trace => "[trace]".apply_style(styles::TRACE_LOG),
        };
        let string = format!("{}", args = record.args());
        let indented = string
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let mut indent = "";
                let mut new_line = "";
                if i > 0 {
                    indent = "  ";
                    new_line = "\n"
                }
                format!("{new_line}{level} {indent}{line}")
            })
            .collect::<String>();
        writeln!(buf, "{}", indented)
    };

    builder
        .format(log_format)
        .filter(None, log::LevelFilter::Info)
        .init();

    let mut app = command!()
        .name(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .subcommand(
            Command::new("add").about("Downloads the given git repo as a dot").arg(
                arg!(REPO: "A git url that points to a Dot repo containing all your dotfiles")
                    .required(true),
            ).arg(arg!(overwrite: --overwrite "Will remove pre-existing packages of the same name")),
        )
        .subcommand(Command::new("install")
            .about("Installs all Dots")
            .arg(arg!(REPO: "An optional git url that points to a Dot repo that you want to add before installing"))
            .arg(arg!(overwrite: --overwrite "Will remove pre-existing dots of the same name"))
            .arg(arg!(force: -f --force "Will remove pre-existing directories when creating symlinks"))
            .arg(arg!(dry: --dry "Run through the install plan without actually making any changes"))
        )
        .subcommand(Command::new("list")
            .alias("ls")
            .about("List the names of all installed dots")
            .arg(arg!(origins: --origins "List the git origin of each dot"))
       )
        .subcommand(Command::new("path")
            .about("Returns the installed location of a given dot")
            .arg(arg!(DOT: "The dot package name that you would like to search for").required(true))
       );

    let usage = app.render_usage();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => commands::add(sub_matches),
        Some(("install", sub_matches)) => commands::install(sub_matches),
        Some(("list", sub_matches)) => commands::list(sub_matches),
        Some(("path", sub_matches)) => commands::path(sub_matches),
        _ => println!("{}", usage),
    }
}
