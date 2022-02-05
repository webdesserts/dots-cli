mod subcommand_add {
    use assert_cmd::Command;
    use std::fs;
    use test_utils::{AssertableOutput, Fixture, TestDir, TestResult};
    use utils::git::commit_all;

    #[test]
    fn it_should_add_a_dot_to_the_dots_folder() -> TestResult {
        let test_dir = TestDir::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = test_dir.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = test_dir.dots_root();

        let mut cmd = Command::cargo_bin("dots")?;

        cmd.arg("add")
            .arg(&fixture_path)
            .arg("--dotsPath")
            .arg(&dots_root);

        let output = cmd.output()?;
        let expected = format!(
            std::include_str!("output/add_success.out"),
            SRC_PATH = fixture_path,
            DEST_PATH = dots_root.join(fixture.name()),
        );

        output.assert_stderr_eq(expected).assert_success();

        assert!(dots_root.exists());
        assert!(fixture_path.exists());
        assert!(fixture_path.join("Dot.toml").exists());
        Ok(())
    }

    #[test]
    fn it_should_complain_if_there_is_no_dot_toml() -> TestResult {
        let test_dir = TestDir::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = test_dir.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = test_dir.dots_root();
        let dot_toml_path = fixture_path.join("Dot.toml");

        // remove Dot.toml from fixture copy
        fs::remove_file(&dot_toml_path)?;
        commit_all(&fixture_path, "remove Dot.toml")?;

        let mut cmd = Command::cargo_bin("dots")?;

        cmd.arg("add")
            .arg(&fixture_path)
            .arg("--dotsPath")
            .arg(&dots_root);

        let output = cmd.output()?;
        let expected = format!(
            std::include_str!("output/add_fail_with_missing_dot_toml.out"),
            SRC_PATH = fixture_path,
        );

        output.assert_stderr_eq(expected).assert_fail_with_code(1);

        Ok(())
    }

    #[test]
    fn it_should_fail_if_the_dot_is_already_added() -> TestResult {
        let test_dir = TestDir::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = test_dir.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = test_dir.dots_root();

        let mut first_add_cmd = Command::cargo_bin("dots")?;

        first_add_cmd
            .arg("add")
            .arg(&fixture_path)
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let mut second_add_cmd = Command::cargo_bin("dots")?;

        let output = second_add_cmd
            .arg("add")
            .arg(&fixture_path)
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let expected = format!(
            std::include_str!("output/add_fail_with_overwrite_warning.out"),
            SRC_PATH = fixture_path,
            DEST_PATH = dots_root.join(fixture.name()),
        );

        output.assert_stderr_eq(expected).assert_fail_with_code(1);

        Ok(())
    }

    #[test]
    fn it_should_succeed_if_the_dot_is_already_added_but_overwrite_is_passed() -> TestResult {
        let test_dir = TestDir::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = test_dir.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = test_dir.dots_root();

        let mut first_add_cmd = Command::cargo_bin("dots")?;

        first_add_cmd
            .arg("add")
            .arg(&fixture_path)
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let mut second_add_cmd = Command::cargo_bin("dots")?;

        let output = second_add_cmd
            .arg("add")
            .arg(&fixture_path)
            .arg("--overwrite")
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let expected = format!(
            std::include_str!("output/add_success_using_overwrite.out"),
            SRC_PATH = fixture_path,
            DEST_PATH = dots_root.join(fixture.name()),
        );

        output.assert_stderr_eq(expected).assert_success();

        Ok(())
    }

    #[test]
    fn it_should_complain_if_no_repo_repo_was_passed() -> TestResult {
        let test_dir = TestDir::new()?;
        let dots_root = test_dir.dots_root();

        let mut cmd = Command::cargo_bin("dots")?;

        let output = cmd.arg("add").arg("--dotsPath").arg(&dots_root).output()?;

        let expected = std::include_str!("output/add_fail_with_missing_repo.out").to_string();

        output.assert_stderr_eq(expected).assert_fail_with_code(1);

        Ok(())
    }

    #[test]
    fn it_should_show_help_when_help_flag_is_passed() -> TestResult {
        let test_dir = TestDir::new()?;
        let dots_root = test_dir.dots_root();

        let mut cmd = Command::cargo_bin("dots")?;

        let output = cmd
            .arg("add")
            .arg("--help")
            .arg("--dotsPath")
            .arg(&dots_root)
            .output()?;

        let expected = std::include_str!("output/add_help.out").to_string();

        output.assert_stdout_eq(expected).assert_success();

        Ok(())
    }
}
