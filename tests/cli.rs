mod cli_tests {
    use assert_cmd::prelude::*;
    use std::process::Command;
    use test_utils::{AssertableOutput, Fixture, TestDir};

    type TestResult = Result<(), failure::Error>;

    mod root_command {
        use crate::cli_tests::*;

        #[test]
        fn it_should_print_usage() -> TestResult {
            let mut cmd = Command::cargo_bin("dots")?;
            let output = cmd.output()?;
            let expected = include_str!("output/usage.out");

            output.assert_success().assert_stdout_eq(expected);
            Ok(())
        }
    }

    mod help_subcommand {
        use crate::cli_tests::*;

        #[test]
        fn it_should_print_help() -> TestResult {
            let mut cmd = Command::cargo_bin("dots")?;
            let output = cmd.arg("help").output()?;
            let expected = include_str!("output/help.out");

            output.assert_success().assert_stdout_eq(expected);
            Ok(())
        }
    }

    mod list_subcommand {
        use crate::cli_tests::*;

        #[test]
        fn it_should_list_nothing_by_default() -> TestResult {
            let test_dir = TestDir::new()?;
            let mut cmd = Command::cargo_bin("dots").unwrap();
            let output = cmd
                .arg("list")
                .arg("--dotsPath")
                .arg(test_dir.dots_root())
                .output()?;

            output.assert_success().assert_stdout_eq("");
            Ok(())
        }
    }

    mod add_subcommand {
        use crate::cli_tests::*;

        #[test]
        /**
         * @todo consider removing git output from stderr unless there was an error
         */
        fn it_should_add_a_dot_to_the_dots_folder() -> TestResult {
            let test_dir = TestDir::new()?;
            let fixture = Fixture::ExampleDot;
            let fixture_path = test_dir.setup_fixture_as_git_repo(&fixture)?;
            let dots_root = test_dir.dots_root();

            let mut cmd = Command::cargo_bin("dots").unwrap();

            cmd.arg("add")
                .arg(&fixture_path)
                .arg("--dotsPath")
                .arg(&dots_root);

            let output = cmd.output()?;
            let expected = format!(
                std::include_str!("output/add_dot_success.out"),
                SRC_PATH = fixture_path,
                DEST_PATH = dots_root.join(fixture.name()),
            );

            output.assert_success().assert_stderr_eq(expected);

            assert!(dots_root.exists());
            assert!(fixture_path.exists());
            assert!(fixture_path.join("Dot.toml").exists());
            Ok(())
        }
    }
}
