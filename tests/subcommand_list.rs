mod subcommand_list {
    use test_utils::{cargo_bin, AssertableOutput, Fixture, TestManager, TestResult};

    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn it_should_print_help_text_if_help_flag_is_passed() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("list").arg("--help").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(include_str!("output/list_help.out"))
            .assert_success();

        Ok(())
    }

    #[test]
    fn it_should_list_nothing_by_default() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("list").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq("")
            .assert_success();

        Ok(())
    }

    #[test]
    fn it_should_list_multiple_dots_if_multiple_are_added() -> TestResult {
        let fixture1 = Fixture::ExampleDot;
        let fixture2 = Fixture::ExampleDotWithDirectory;
        let manager = TestManager::new()?;
        let fixture1_path = manager.setup_fixture_as_git_repo(&fixture1)?;
        let fixture2_path = manager.setup_fixture_as_git_repo(&fixture2)?;

        manager.cmd(BIN)?.arg("add").arg(&fixture1_path).output()?;
        manager.cmd(BIN)?.arg("add").arg(&fixture2_path).output()?;

        let output = manager.cmd(BIN)?.arg("list").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(include_str!(
                "output/list_success_with_multiple_directories.out"
            ))
            .assert_success();

        Ok(())
    }

    #[test]
    fn it_should_also_work_with_the_ls_shorthand_alias() -> TestResult {
        let fixture1 = Fixture::ExampleDot;
        let fixture2 = Fixture::ExampleDotWithDirectory;
        let manager = TestManager::new()?;
        let fixture1_path = manager.setup_fixture_as_git_repo(&fixture1)?;
        let fixture2_path = manager.setup_fixture_as_git_repo(&fixture2)?;

        manager.cmd(BIN)?.arg("add").arg(&fixture1_path).output()?;
        manager.cmd(BIN)?.arg("add").arg(&fixture2_path).output()?;

        let output = manager.cmd(BIN)?.arg("ls").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(include_str!(
                "output/list_success_with_multiple_directories.out"
            ))
            .assert_success();

        Ok(())
    }

    #[test]
    fn it_should_print_the_git_origin_of_each_dot_if_the_origins_flag_is_passed() -> TestResult {
        let fixture1 = Fixture::ExampleDot;
        let fixture2 = Fixture::ExampleDotWithDirectory;
        let manager = TestManager::new()?;
        let fixture1_path = manager.setup_fixture_as_git_repo(&fixture1)?;
        let fixture2_path = manager.setup_fixture_as_git_repo(&fixture2)?;

        manager.cmd(BIN)?.arg("add").arg(&fixture1_path).output()?;
        manager.cmd(BIN)?.arg("add").arg(&fixture2_path).output()?;

        let output = manager.cmd(BIN)?.arg("list").arg("--origins").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(format!(
                include_str!("output/list_success_with_origins_flag.out"),
                FIXTURE1_PATH = &fixture1_path,
                FIXTURE2_PATH = &fixture2_path,
            ))
            .assert_success();

        Ok(())
    }
}
