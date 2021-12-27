mod command_dots {
    use assert_cmd::Command;
    use test_utils::{AssertableOutput, TestResult};

    #[test]
    fn it_should_print_usage() -> TestResult {
        let mut cmd = Command::cargo_bin("dots")?;
        let output = cmd.output()?;
        let expected = include_str!("output/usage.out");

        output.assert_success().assert_stdout_eq(expected);
        Ok(())
    }
}
