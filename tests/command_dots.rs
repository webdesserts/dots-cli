mod command_dots {
    use clap::crate_version;
    use test_utils::{cargo_bin, AssertableOutput, TestManager, TestResult};
    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn it_should_print_usage() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.output()?;
        let expected = include_str!("output/usage.out");

        output.assert_stdout_eq(expected).assert_success();
        Ok(())
    }

    #[test]
    fn it_should_print_help_if_the_help_command_is_passed() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("--help").output()?;
        let expected = format!(include_str!("output/help.out"), VERSION = crate_version!());

        output.assert_stdout_eq(expected).assert_success();
        Ok(())
    }

    #[test]
    fn it_should_print_the_version_if_the_version_flag_is_passed() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("--version").output()?;
        let expected = format!(
            include_str!("output/version.out"),
            VERSION = crate_version!()
        );

        output.assert_stdout_eq(expected).assert_success();
        Ok(())
    }
}
