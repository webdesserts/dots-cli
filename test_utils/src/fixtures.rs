use camino::Utf8PathBuf;
use std::fmt::Display;
use utils::fs::current_dir;

pub enum Fixture {
    ExampleDot,
}

impl Fixture {
    /** The path where fixture templates can be found */
    pub fn templates_root() -> Utf8PathBuf {
        current_dir().join("fixtures")
    }

    /** The name of this fixture's package and containing folder */
    pub fn name(&self) -> &str {
        match self {
            Self::ExampleDot => "example_dot",
        }
    }

    /** The path where this specific fixture's template can be found */
    pub fn template_path(&self) -> Utf8PathBuf {
        Self::templates_root().join(self.name())
    }
}

impl Display for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
