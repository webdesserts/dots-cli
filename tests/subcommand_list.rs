mod subcommand_list {
    use assert_cmd::Command;
    use test_utils::{AssertableOutput, TestManager, TestResult};

    #[test]
    fn it_should_list_nothing_by_default() -> TestResult {
        let manager = TestManager::new()?;
        let mut cmd = Command::cargo_bin("dots")?;
        let output = cmd
            .arg("list")
            .arg("--dotsPath")
            .arg(manager.dots_dir())
            .output()?;

        output.assert_stdout_eq("").assert_success();
        Ok(())
    }
}
