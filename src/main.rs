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
mod footprint;
mod fs_manager;
pub mod plan;

use std::io::Write;

use clap::Parser;
use env_logger::fmt::Formatter;
use env_logger::Builder;
use utils::stylize::Style;

mod styles {
    use utils::stylize::Style;

    const LOG: Style = Style::new().bold();

    pub const DEBUG_LOG: Style = LOG;
    pub const INFO_LOG: Style = LOG.blue();
    pub const WARN_LOG: Style = LOG.yellow();
    pub const ERROR_LOG: Style = LOG.red();
    pub const TRACE_LOG: Style = LOG;
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    commands: Option<Commands>,
    /// prints extra logs for debugging purposes
    #[clap(long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Downloads the given git repo as a dot
    Add {
        /// A git url that points to a Dot repo containing all your dotfiles
        repo: String,
        /// Will remove pre-existing packages of the same name
        #[clap(long)]
        overwrite: bool,
    },

    /// Downloads and links dots
    Install {
        /// An optional git url that points to a Dot repo that you want to add before installing
        repo: Option<String>,

        /// Will remove pre-existing dots of the same name
        #[clap(long)]
        overwrite: bool,

        /// Will remove pre-existing directories when creating symlinks
        #[clap(short, long)]
        force: bool,

        /// Run through the install plan without actually making any changes
        #[clap(long)]
        dry: bool,
    },

    /// Removes and unlinks dots
    Uninstall {
        /// The name of the dot you'd like to remove
        dot_name: Option<String>,
    },

    /// List the names of all installed dots
    #[clap(alias = "ls")]
    List {
        /// List the git origin of each dot
        #[clap(long)]
        origins: bool,
    },

    /// Get the current git status of each dot
    Status,

    /// Returns the installed location of a given dot
    Path {
        /// The dot package name that you would like to search for
        dot: String,
    },
}

fn main() {
    Style::detect_color_support();
    let mut builder = Builder::new();

    let log_format = |buf: &mut Formatter, record: &log::Record| -> Result<(), std::io::Error> {
        use log::Level::*;
        let level = match record.level() {
            Debug => styles::DEBUG_LOG.apply("[debug]"),
            Info => styles::INFO_LOG.apply("[info]"),
            Warn => styles::WARN_LOG.apply("[warn]"),
            Error => styles::ERROR_LOG.apply("[error]"),
            Trace => styles::TRACE_LOG.apply("[trace]"),
        };
        let string = format!("{args}", args = record.args());
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

    let cli = Cli::parse();

    let log_level = if cli.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    builder.format(log_format).filter(None, log_level).init();

    match &cli.commands {
        Some(Commands::Add { repo, overwrite }) => commands::add(repo, *overwrite),
        Some(Commands::Install {
            repo,
            overwrite,
            force,
            dry,
        }) => commands::install(repo, *overwrite, *force, *dry),
        Some(Commands::Uninstall { dot_name }) => commands::uninstall(dot_name),
        Some(Commands::List { origins }) => commands::list(*origins),
        Some(Commands::Status) => commands::status(),
        Some(Commands::Path { dot }) => commands::path(dot),
        _ => {
            println!("USAGE:\n    dots [SUBCOMMAND]")
        }
    }
}
