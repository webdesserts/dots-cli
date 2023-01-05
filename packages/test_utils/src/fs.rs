use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use utils::stylize::Style;

use crate::indent;

mod styles {
    use utils::stylize::Style;
    pub const PATH: Style = Style::new().dim();
    pub const DIR: Style = Style::new().bold();
    pub const LINK: Style = Style::new().underlined();

    pub const EOF: Style = Style::new().dim();
}

pub fn ls<P: AsRef<Utf8Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    println!();
    println!("{}", styles::PATH.apply(path.as_str()));

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = Utf8PathBuf::try_from(entry.path())?;
        let mut style = Style::new();

        if path.is_dir() {
            style = style + styles::DIR;
        }
        if path.is_symlink() {
            style = style + styles::LINK;
        }

        if let Some(path) = path.file_name() {
            println!("{}", style.apply(indent(2, path)));
        }
    }

    println!();
    Ok(())
}

pub fn cat<P: AsRef<Utf8Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    let content = fs::read_to_string(path)?;
    println!();
    println!("{}", styles::PATH.apply(path.as_str()));
    println!("{}", indent(2, content));
    println!("{}", styles::EOF.apply("EOF"));
    println!();

    Ok(())
}
