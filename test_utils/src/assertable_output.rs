use similar::{Algorithm, ChangeTag, TextDiff};
use std::fmt::Write;
use std::process::Output;
use utils::stylize::Stylable;

mod styles {
    use utils::stylize::Style;

    pub const EXPECTED: Style = Style::new().green();
    pub const RECEIVED: Style = Style::new().red();

    pub const NORMAL: Style = Style::new().dim();
    pub const EMPHASIS: Style = Style::new().reverse().bold();

    pub const SIGN: Style = Style::new().bold();
    pub const SEPARATOR: Style = Style::new().italic().dim();
}

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
        &self
    }

    fn assert_fail(&self) -> &Self {
        assert!(
            !self.status.success(),
            "expected command to fail, but it succeeded"
        );
        &self
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

        &self
    }
}

fn pretty_assert<E, A>(expected: E, actual: A)
where
    E: AsRef<str>,
    A: AsRef<str>,
{
    let expected = expected.as_ref();
    let actual = actual.as_ref();

    if expected != actual {
        let diff = TextDiff::configure()
            .algorithm(Algorithm::Patience)
            .diff_lines(expected, actual);

        let diff_text: String = diff
            .grouped_ops(5)
            .into_iter()
            .enumerate()
            .map(|(index, group)| {
                let group_sep = if index > 0 { "@@ ~~~\n" } else { "" };

                let group_diff: String = group
                    .into_iter()
                    .map(|op| {
                        diff.iter_inline_changes(&op)
                            .map(|change| {
                                let (sign, style) = match change.tag() {
                                    ChangeTag::Equal => (" ", styles::NORMAL),
                                    ChangeTag::Delete => ("-", styles::EXPECTED),
                                    ChangeTag::Insert => ("+", styles::RECEIVED),
                                };

                                let mut line: String = String::from("");

                                let sign_style = style.merge(&styles::SIGN);

                                write!(line, "{} ", sign.apply_style(sign_style)).ok();

                                for &(emphasis, value) in change.values() {
                                    let mut value_style = style.clone();

                                    if emphasis {
                                        value_style = value_style.merge(&styles::EMPHASIS);
                                    };

                                    write!(line, "{}", value.apply_style(value_style)).ok();
                                }

                                if change.missing_newline() {
                                    writeln!(line, "{}", "↵".apply_style(styles::EXPECTED.bold()))
                                        .ok();
                                }

                                line
                            })
                            .collect::<String>()
                    })
                    .collect();
                format!(
                    "{}{}",
                    &group_sep.apply_style(styles::SEPARATOR),
                    group_diff
                )
            })
            .collect();

        let received_label = "Received ".apply_style(styles::RECEIVED);
        let expected_label = "Expected ".apply_style(styles::EXPECTED);
        let legend = format!(
            "{expected_sign} {expected}\n{received_sign} {received}",
            expected_sign = "-".apply_style(styles::EXPECTED.merge(&styles::SIGN)),
            received_sign = "+".apply_style(styles::RECEIVED.merge(&styles::SIGN)),
            expected = expected_label,
            received = received_label
        );

        println!(
            "\n{legend}\n\n{diff}\n",
            legend = legend,
            diff = indent(2, diff_text)
        );
        panic!("assertion failed")
    }
}

fn indent<S>(indent: usize, string: S) -> String
where
    S: AsRef<str>,
{
    let string = string.as_ref();
    string
        .lines()
        .map(|line: &str| format!("{indent}{line}\n", indent = " ".repeat(indent), line = line,))
        .collect()
}
