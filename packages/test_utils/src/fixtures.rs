use camino::Utf8PathBuf;
use std::fmt::Display;
use utils::fs::current_dir;

pub enum Fixture {
    ExampleDot,
    ExampleDotWithLinkAdded,
    ExampleDotWithDirectory,
    ConflictingDot,
}

impl Fixture {
    /** The path where fixture templates can be found */
    pub fn templates_root() -> Utf8PathBuf {
        current_dir().join("fixtures")
    }

    /** The name of this fixture's package and containing folder */
    pub fn name(&self) -> &str {
        match self {
            Self::ExampleDotWithDirectory => "example_dot_with_directory",
            Self::ExampleDotWithLinkAdded => "example_dot",
            Self::ExampleDot => "example_dot",
            Self::ConflictingDot => "conflicting_dot",
        }
    }

    /** The path where this specific fixture's template can be found */
    pub fn template_path(&self) -> Utf8PathBuf {
        match self {
            Self::ExampleDotWithLinkAdded => {
                Self::templates_root().join("example_dot_with_link_added")
            }
            _ => Self::templates_root().join(self.name()),
        }
    }
}

impl Display for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
