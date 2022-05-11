use crate::pretty_assert;
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
    fn assert_fail_with_code(&self, code: i32) -> &Self;
}

impl AssertableOutput for Output {
    fn assert_stdout_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>,
    {
        let expected = expected.as_ref();

        let stdout = self.stdout.clone();
        let stdout_str = std::str::from_utf8(&stdout).unwrap();

        pretty_assert(expected, stdout_str);

        self
    }

    fn assert_stderr_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>,
    {
        let expected = expected.as_ref();

        let stderr = self.stderr.clone();
        let stderr_str = std::str::from_utf8(&stderr).unwrap();

        pretty_assert(expected, stderr_str);
        self
    }

    fn assert_success(&self) -> &Self {
        assert!(
            self.status.success(),
            "expected command to succeed, but it failed with code {:?}",
            self.status.code()
        );
        self
    }

    fn assert_fail(&self) -> &Self {
        assert!(
            !self.status.success(),
            "expected command to fail, but it succeeded"
        );
        self
    }

    fn assert_fail_with_code(&self, expected_code: i32) -> &Self {
        match self.status.code() {
            Some(0) => panic!("expected command to fail, but it succeeded"),
            Some(code) => assert_eq!(
                expected_code, code,
                "expected command to fail with exit code {} but instead it failed with {}",
                expected_code, code
            ),
            None => panic!(
                "expected command to fail with exit code {} but the process was terminated early",
                expected_code
            ),
        }

        self
    }
}
