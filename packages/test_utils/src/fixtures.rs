use camino::Utf8PathBuf;
use std::fmt::Display;
use utils::fs::current_dir;

pub enum Fixture {
    ExampleDot,
    ExampleDotWithUnlinkedFile,
    ExampleDotWithLinkAdded,
    ExampleDotWithMultiLink,
    ExampleDotWithDirectory,
    ExampleDotWithNestedDirectories,
    ConflictingDot,
    ExampleDotWithSelfLink,
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
            Self::ExampleDotWithNestedDirectories => "example_dot",
            Self::ExampleDotWithLinkAdded => "example_dot",
            Self::ExampleDotWithMultiLink => "example_dot",
            Self::ExampleDotWithUnlinkedFile => "example_dot",
            Self::ExampleDot => "example_dot",
            Self::ConflictingDot => "conflicting_dot",
            Self::ExampleDotWithSelfLink => "example_dot_with_self_link",
        }
    }

    /** The path where this specific fixture's template can be found */
    pub fn template_path(&self) -> Utf8PathBuf {
        let subpath = match self {
            Self::ExampleDotWithLinkAdded => "example_dot_with_link_added",
            Self::ExampleDotWithMultiLink => "example_dot_with_multi_link",
            Self::ExampleDotWithUnlinkedFile => "example_dot_with_unlinked_file",
            Self::ExampleDotWithNestedDirectories => "example_dot_with_nested_directories",
            _ => self.name(),
        };
        Self::templates_root().join(subpath)
    }
}

impl Display for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
