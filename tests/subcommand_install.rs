mod subcommand_install {
    use assert_cmd::Command;
    use test_utils::{AssertableOutput, Fixture, TestManager, TestResult};

    #[test]
    fn it_should_display_the_install_plan_but_not_install_if_the_dry_option_is_passed() -> TestResult
    {
        let manager = TestManager::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = manager.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = manager.dots_dir();

        let mut cmd = Command::cargo_bin("dots")?;
        cmd.arg("add")
            .arg(&fixture_path)
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let mut cmd = Command::cargo_bin("dots")?;
        let output = cmd
            .arg("install")
            .arg("--dry")
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let expected_err = std::include_str!("output/install_success_with_dry.err");
        let expected_out = std::include_str!("output/install_success_with_dry.out");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq(expected_out)
            .assert_fail_with_code(1);

        assert!(dots_root.exists());
        assert!(fixture_path.exists());
        assert!(fixture_path.join("Dot.toml").exists());
        Ok(())
    }
}
