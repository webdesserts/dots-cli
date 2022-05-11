use similar::{Algorithm, ChangeTag, TextDiff};
use std::fmt::Write;
use utils::stylize::Stylable;

mod styles {
    use utils::{style, stylize::Style};

    pub const EXPECTED: Style = style! { color: Green };
    pub const RECEIVED: Style = style! { color: Red };

    pub const NORMAL: Style = style! { Dim };
    pub const EMPHASIS: Style = style! { Reverse; Bold };

    pub const SIGN: Style = style! { Bold };
    pub const SEPARATOR: Style = style! { Italic; Dim };
}

pub fn pretty_assert<E, A>(expected: E, actual: A)
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
                                    let mut value_style = style;

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
                    "{separator}{group_diff}",
                    separator = &group_sep.apply_style(styles::SEPARATOR),
                )
            })
            .collect();

        let received_label = "Received ".apply_style(styles::RECEIVED);
        let expected_label = "Expected ".apply_style(styles::EXPECTED);
        let expected_sign = "-".apply_style(styles::EXPECTED.merge(&styles::SIGN));
        let received_sign = "+".apply_style(styles::RECEIVED.merge(&styles::SIGN));

        let legend = format!("{expected_sign} {expected_label}\n{received_sign} {received_label}");

        println!("\n{legend}\n\n{diff}\n", diff = indent(2, diff_text));
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
        .map(|line: &str| format!("{indent}{line}\n", indent = " ".repeat(indent)))
        .collect()
}
