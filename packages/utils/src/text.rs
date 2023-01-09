/// takes the given string and indents every line by the given number of spaces
pub fn indent(spaces: usize, string: &str) -> String {
    let has_extra_newline = string.ends_with('\n');
    let lines: Vec<String> = string
        .lines()
        .map(|line| format!("{spaces}{line}", spaces = " ".repeat(spaces)))
        .collect();

    let string = lines.join("\n");

    if has_extra_newline {
        format!("{string}\n")
    } else {
        string
    }
}
