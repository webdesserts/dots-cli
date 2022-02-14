mod subcommand_list {
    use test_utils::{cargo_bin, AssertableOutput, TestManager, TestResult};

    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn it_should_list_nothing_by_default() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager
            .cmd(BIN)?
            .arg("list")
            .arg("--dotsPath")
            .arg(manager.dots_dir())
            .output()?;

        output.assert_stdout_eq("").assert_success();
        Ok(())
    }
}
