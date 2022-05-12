use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use utils::stylize::{Stylable, Style};

use crate::indent;

mod styles {
    use utils::{style, stylize::Style};
    pub const PATH: Style = style! { Dim };
    pub const DIR: Style = style! { Bold };
    pub const LINK: Style = style! { Underlined };

    pub const EOF: Style = style! { Dim };
}

pub fn ls<P: AsRef<Utf8Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    println!();
    println!("{}", path.as_str().apply_style(styles::PATH));

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = Utf8PathBuf::try_from(entry.path())?;
        let mut style = Style::new();

        if path.is_dir() {
            style = style.merge(&styles::DIR);
        }
        if path.is_symlink() {
            style = style.merge(&styles::LINK);
        }

        if let Some(path) = path.file_name() {
            println!("{}", indent(2, path).apply_style(style));
        }
    }

    println!();
    Ok(())
}

pub fn cat<P: AsRef<Utf8Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    let content = fs::read_to_string(path)?;
    println!();
    println!("{}", path.as_str().apply_style(styles::PATH));
    println!("{}", indent(2, content));
    println!("{}", "EOF".apply_style(styles::EOF));
    println!();

    Ok(())
}
