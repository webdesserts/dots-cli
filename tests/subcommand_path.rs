mod subcommand_path {
    use test_utils::{cargo_bin, AssertableOutput, Fixture, TestManager, TestResult};

    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn it_should_print_help_when_the_help_flag_is_passed() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("path").arg("--help").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(include_str!("output/path_help.out"))
            .assert_success();
        Ok(())
    }

    #[test]
    fn it_should_print_usage_when_its_missing_arguments() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("path").output()?;

        output
            .assert_stderr_eq(include_str!("output/path_fail_with_missing_dot.out"))
            .assert_stdout_eq("")
            .assert_fail_with_code(2);
        Ok(())
    }

    #[test]
    fn it_should_return_nothing_if_the_dot_doesnt_exist() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("path").arg("missing_dot").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq("")
            .assert_fail_with_code(1);
        Ok(())
    }

    #[test]
    fn it_should_return_the_path_of_the_given_dot_if_it_is_added() -> TestResult {
        let fixture = Fixture::ExampleDot;
        let manager = TestManager::new()?;
        let fixture_path = manager.setup_fixture_as_git_repo(&fixture)?;
        let dot_path = manager.dots_dir().join(fixture.name());

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("path").arg(fixture.name()).output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(&dot_path)
            .assert_success();
        Ok(())
    }
}
