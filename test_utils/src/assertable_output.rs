use pretty_assertions::assert_eq;
use std::process::Output;

pub trait AssertableOutput {
    fn assert_stderr_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>;
    fn assert_stdout_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>;
    fn assert_success(&self) -> &Self;
    fn assert_fail(&self) -> &Self;
    fn assert_fail_with_signal(&self, signal: i32) -> &Self;
}

impl AssertableOutput for Output {
    fn assert_stdout_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>,
    {
        let expected = expected.as_ref();

        let stdout = self.stdout.clone();
        let stdout_str = std::str::from_utf8(&stdout).unwrap();

        assert_eq!(stdout_str, expected);

        self
    }

    fn assert_stderr_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>,
    {
        let expected = expected.as_ref();

        let stderr = self.stderr.clone();
        let stderr_str = std::str::from_utf8(&stderr).unwrap();

        assert_eq!(stderr_str, expected);
        self
    }

    fn assert_success(&self) -> &Self {
        assert!(
            self.status.success(),
            "expected command to succeed, but it failed with code {:?}",
            self.status.code()
        );
        &self
    }

    fn assert_fail(&self) -> &Self {
        assert!(
            !self.status.success(),
            "expected command to fail, but it succeeded"
        );
        &self
    }

    fn assert_fail_with_signal(&self, expected_code: i32) -> &Self {
        let code = self.status.code();
        assert_eq!(
            code,
            Some(expected_code),
            "expected fail signal {:?} but it succeeded with {:?}",
            expected_code,
            code
        );
        &self
    }
}
