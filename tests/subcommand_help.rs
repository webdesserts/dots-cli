mod subcommand_help {
    use assert_cmd::Command;
    use clap::crate_version;
    use test_utils::{AssertableOutput, TestResult};

    #[test]
    fn it_should_print_help() -> TestResult {
        let mut cmd = Command::cargo_bin("dots")?;
        let output = cmd.arg("help").output()?;
        let expected = format!(include_str!("output/help.out"), VERSION = crate_version!());

        output.assert_stdout_eq(expected).assert_success();
        Ok(())
    }
}
