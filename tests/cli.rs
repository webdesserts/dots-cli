const VERSION: &str = "dots 0.2.1";
const AUTHOR: &str = "Michael Mullins <michael@webdesserts.com>";
const DESCRIPTION: &str = "A cli for managing all your dot(file)s";

const USAGE: &str = "USAGE:
    dots [OPTIONS] [SUBCOMMAND]";

const FLAGS: &str = "FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

const OPTIONS: &str = "OPTIONS:
        --dotsPath <dots_path>    The location to find and install dots";

const SUBCOMMANDS: &str = "SUBCOMMANDS:
    add        Downloads the given git repo as a dot
    help       Prints this message or the help of the given subcommand(s)
    install    Installs all Dots
    list       List the names of all installed dots and the repos they link to
    prefix     returns the installed location of a given dot";

mod cli_tests {
    use crate::*;
    use assert_cmd::prelude::*;
    use std::process::Command;
    
    type TestResult = Result<(), failure::Error>;

    mod root_command {
        use crate::cli_tests::*;

        #[test]
        fn it_should_print_usage() {
            let mut cmd = Command::cargo_bin("dots").unwrap();
            cmd.assert().success().stdout(format!("{}\n", USAGE));
        }
    }

    mod help_subcommand {
        use crate::cli_tests::*;

        #[test]
        fn it_should_print_help() {
            let mut cmd = Command::cargo_bin("dots").unwrap();
            cmd.arg("help").assert().success().stdout(format!(
                "{}\n{}\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n",
                VERSION, AUTHOR, DESCRIPTION, USAGE, FLAGS, OPTIONS, SUBCOMMANDS
            ));
        }
    }

    mod list_subcommand {
        use crate::cli_tests::*;
        use tempfile::tempdir;

        #[test]
        fn it_should_list_nothing_by_default() -> TestResult {
            let dir = tempdir()?;
            let mut cmd = Command::cargo_bin("dots").unwrap();
            cmd.arg("list")
                .arg("--dotsPath")
                .arg(dir.path())
                .assert()
                .success()
                .stdout("");
            Ok(())
        }
    }
}
