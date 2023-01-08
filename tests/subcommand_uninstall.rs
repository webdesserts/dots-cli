mod subcommand_uninstall {
    use test_utils::{cargo_bin, AssertableOutput, Fixture, TestManager, TestResult};
    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn should_print_help_text_when_the_help_flag_is_passed() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("uninstall").arg("--help").output()?;

        output
            .assert_stderr_eq("")
            .assert_stdout_eq(include_str!("output/uninstall_help.out"))
            .assert_success();

        Ok(())
    }

    #[test]
    fn should_remove_the_given_dot_when_its_name_is_passed_as_an_arg() -> TestResult {
        let manager = TestManager::new()?;
        let fixture1 = Fixture::ExampleDot;
        let fixture2 = Fixture::ExampleDotWithDirectory;
        let fixture1_path = manager.setup_fixture_as_git_repo(&fixture1)?;
        let fixture2_path = manager.setup_fixture_as_git_repo(&fixture2)?;

        let dot1_path = manager.expected_dot_path(&fixture1);
        let dot2_path = manager.expected_dot_path(&fixture2);

        manager
            .cmd(BIN)?
            .arg("install")
            .arg(fixture1_path)
            .output()?;

        manager
            .cmd(BIN)?
            .arg("install")
            .arg(fixture2_path)
            .output()?;

        manager
            .cmd(BIN)?
            .arg("uninstall")
            .arg(fixture1.name())
            .output()?
            .assert_success();

        assert!(!dot1_path.exists());
        assert!(dot2_path.exists());

        Ok(())
    }

    #[test]
    fn should_remove_links_from_the_given_dot() -> TestResult {
        let manager = TestManager::new()?;
        let fixture1 = Fixture::ExampleDot;
        let fixture2 = Fixture::ExampleDotWithDirectory;
        let fixture1_path = manager.setup_fixture_as_git_repo(&fixture1)?;
        let fixture2_path = manager.setup_fixture_as_git_repo(&fixture2)?;
        let home_path = manager.home_dir();

        manager
            .cmd(BIN)?
            .arg("install")
            .arg(&fixture1_path)
            .output()?;

        manager
            .cmd(BIN)?
            .arg("install")
            .arg(fixture2_path)
            .output()?;

        manager
            .cmd(BIN)?
            .arg("uninstall")
            .arg(fixture1_path)
            .output()?;

        assert!(!home_path.join("./bashrc").exists());
        assert!(!home_path.join("./zshrc").exists());

        assert!(home_path.join("./bin").exists());

        Ok(())
    }

    #[test]
    fn should_throw_and_error_if_there_is_no_dot_with_the_given_name() -> TestResult {
        let manager = TestManager::new()?;
        let fixture1 = Fixture::ExampleDot;
        let dot_name = fixture1.name();

        let output = manager.cmd(BIN)?.arg("uninstall").arg(dot_name).output()?;

        output.assert_fail().assert_stderr_eq(format!(
            include_str!("output/uninstall_fail_with_unkown_dot.err"),
            DOT_NAME = dot_name
        ));

        Ok(())
    }
}
