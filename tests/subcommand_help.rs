mod subcommand_help {
    use clap::crate_version;
    use test_utils::{cargo_bin, AssertableOutput, TestManager, TestResult};
    const BIN: &str = cargo_bin!("dots");

    #[test]
    fn it_should_print_help() -> TestResult {
        let manager = TestManager::new()?;
        let output = manager.cmd(BIN)?.arg("help").output()?;
        let expected = format!(include_str!("output/help.out"), VERSION = crate_version!());

        output.assert_stdout_eq(expected).assert_success();
        Ok(())
    }
}
