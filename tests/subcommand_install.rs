mod subcommand_install {
    use std::{fs, os::unix};
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

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_fail_with_code(1);

        assert!(dots_root.exists());
        assert!(fixture_path.exists());
        assert!(fixture_path.join("Dot.toml").exists());
        Ok(())
    }

    #[test]
    fn it_should_display_and_install_the_given_plan() -> TestResult {
        let manager = TestManager::new()?;
        let fixture = Fixture::ExampleDot;
        let fixture_path = manager.setup_fixture_as_git_repo(&fixture)?;
        let dots_root = manager.dots_dir();
        let home_dir = manager.home_dir();

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("install").output()?;
        let expected_err = std::include_str!("output/install_success.err");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_success();

        let installed_dot_path = dots_root.join(fixture.name());

        assert!(installed_dot_path.is_dir());
        assert!(installed_dot_path.join("Dot.toml").is_file());
        assert_eq!(
            home_dir.join(".bashrc").read_link()?,
            installed_dot_path.join("shell/bashrc")
        );
        assert_eq!(
            home_dir.join(".zshrc").read_link()?,
            installed_dot_path.join("shell/zshrc")
        );
        Ok(())
    }

    #[test]
    fn it_should_fail_if_multiple_dots_have_a_link_the_same_thing() -> TestResult {
        let manager = TestManager::new()?;

        let main_fixture_path = manager.setup_fixture_as_git_repo(&Fixture::ExampleDot)?;
        let conflicting_fixture_path =
            manager.setup_fixture_as_git_repo(&Fixture::ConflictingDot)?;

        let home_dir = manager.home_dir();

        manager
            .cmd(BIN)?
            .arg("add")
            .arg(&main_fixture_path)
            .output()?;

        manager
            .cmd(BIN)?
            .arg("add")
            .arg(&conflicting_fixture_path)
            .output()?;

        let output = manager.cmd(BIN)?.arg("install").output()?;
        let expected_err = std::include_str!("output/install_fail_with_conflicts.err");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_fail_with_code(1);

        assert!(!home_dir.join(".bashrc").exists());
        assert!(!home_dir.join(".zshrc").exists());
        Ok(())
    }

    #[test]
    fn it_should_succeed_if_you_try_to_overwrite_a_symlink() -> TestResult {
        let manager = TestManager::new()?;
        let fixture_path = manager.setup_fixture_as_git_repo(&Fixture::ExampleDot)?;
        let home_dir = manager.home_dir();
        let dot_dir = manager.dots_dir().join(Fixture::ExampleDot.name());

        let old_linked_file_path = home_dir.join("config_bashrc");
        let old_link_path = home_dir.join(".bashrc");

        fs::File::create(&old_linked_file_path)?;

        unix::fs::symlink(&old_linked_file_path, old_link_path)?;

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("install").output()?;
        let expected_err = std::include_str!("output/install_success.err");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_success();

        assert_eq!(
            home_dir.join(".bashrc").read_link()?,
            dot_dir.join("shell/bashrc")
        );
        assert!(home_dir.join(".zshrc").is_symlink());
        Ok(())
    }

    #[test]
    fn it_should_succeed_if_you_try_to_overwrite_a_broken_symlink() -> TestResult {
        let manager = TestManager::new()?;
        let fixture_path = manager.setup_fixture_as_git_repo(&Fixture::ExampleDot)?;
        let home_dir = manager.home_dir();
        let dot_dir = manager.dots_dir().join(Fixture::ExampleDot.name());

        let old_linked_file_path = home_dir.join("config_bashrc");
        let old_link_path = home_dir.join(".bashrc");

        unix::fs::symlink(&old_linked_file_path, old_link_path)?;

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("install").output()?;
        let expected_err = std::include_str!("output/install_success.err");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_success();

        assert_eq!(
            home_dir.join(".bashrc").read_link()?,
            dot_dir.join("shell/bashrc")
        );
        assert!(home_dir.join(".zshrc").is_symlink());
        Ok(())
    }

    #[test]
    fn it_should_fail_if_you_try_to_overwrite_a_dir_and_its_contents() -> TestResult {
        let manager = TestManager::new()?;
        let fixture_path = manager.setup_fixture_as_git_repo(&Fixture::ExampleDot)?;
        let home_dir = manager.home_dir();

        fs::create_dir(home_dir.join(".bashrc"))?;

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("install").output()?;
        let expected_err =
            std::include_str!("output/install_fail_with_existing_directory_warning.err");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_fail_with_code(1);

        assert!(home_dir.join(".bashrc").is_dir());
        assert!(!home_dir.join(".zshrc").exists());
        Ok(())
    }

    #[test]
    fn it_should_succeed_if_you_explicitely_force_an_overwrite_of_an_existing_directory(
    ) -> TestResult {
        let manager = TestManager::new()?;
        let fixture_path = manager.setup_fixture_as_git_repo(&Fixture::ExampleDot)?;
        let home_dir = manager.home_dir();
        let dot_dir = manager.dots_dir().join(Fixture::ExampleDot.name());

        fs::create_dir(home_dir.join(".bashrc"))?;

        manager.cmd(BIN)?.arg("add").arg(&fixture_path).output()?;

        let output = manager.cmd(BIN)?.arg("install").arg("--force").output()?;
        let expected_err =
            std::include_str!("output/install_success_with_existing_directory_and_force.err");

        output
            .assert_stderr_eq(expected_err)
            .assert_stdout_eq("")
            .assert_success();

        assert_eq!(
            home_dir.join(".bashrc").read_link()?,
            dot_dir.join("shell/bashrc")
        );
        assert_eq!(
            home_dir.join(".zshrc").read_link()?,
            dot_dir.join("shell/zshrc")
        );
        Ok(())
    }
}
