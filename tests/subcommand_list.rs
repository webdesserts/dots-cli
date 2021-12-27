mod subcommand_list {
    use assert_cmd::Command;
    use test_utils::{AssertableOutput, TestDir, TestResult};

    #[test]
    fn it_should_list_nothing_by_default() -> TestResult {
        let test_dir = TestDir::new()?;
        let mut cmd = Command::cargo_bin("dots").unwrap();
        let output = cmd
            .arg("list")
            .arg("--dotsPath")
            .arg(test_dir.dots_root())
            .output()?;

        output.assert_success().assert_stdout_eq("");
        Ok(())
    }
}
