mod subcommand_help {
    use assert_cmd::Command;
    use test_utils::{AssertableOutput, TestResult};

    #[test]
    fn it_should_print_help() -> TestResult {
        let mut cmd = Command::cargo_bin("dots")?;
        let output = cmd.arg("help").output()?;
        let expected = include_str!("output/help.out");

        output.assert_success().assert_stdout_eq(expected);
        Ok(())
    }
}
