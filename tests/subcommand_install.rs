mod subcommand_install {
    use test_utils::{cargo_bin, AssertableOutput, Fixture, TestManager, TestResult};

    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn it_should_display_the_install_plan_but_not_install_if_the_dry_option_is_passed() -> TestResult
    {
        let manager = TestManager::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = manager.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = manager.dots_dir();

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("install").arg("--dry").output()?;

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
