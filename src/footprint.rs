use std::collections::BTreeSet;

use crate::plan::links::{Anchor, Link};
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Footprint {
    #[serde(default)]
    pub dirs: BTreeSet<Utf8PathBuf>,
    #[serde(default)]
    pub links: BTreeSet<Link>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FootprintLink {
    /// An absolute path to the dotfile
    pub src: Utf8PathBuf,
    /// An absolute path to the symlink
    pub dest: Utf8PathBuf,
}

impl From<Link> for FootprintLink {
    fn from(link: Link) -> Self {
        FootprintLink {
            src: link.src.path,
            dest: link.dest.path,
        }
    }
}

impl From<&Link> for FootprintLink {
    fn from(link: &Link) -> Self {
        FootprintLink {
            src: link.src.path.clone(),
            dest: link.dest.path.clone(),
        }
    }
}

impl From<FootprintLink> for Link {
    fn from(link: FootprintLink) -> Self {
        Link {
            src: Anchor::new_src(link.src),
            dest: Anchor::new_dest(link.dest),
        }
    }
}

impl From<&FootprintLink> for Link {
    fn from(link: &FootprintLink) -> Self {
        Link {
            src: Anchor::new_src(link.src.clone()),
            dest: Anchor::new_dest(link.dest.clone()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let result = FootprintLink::deserialize(deserializer)?;
        Ok(result.into())
    }
}
impl serde::Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let footprint_link: FootprintLink = self.into();
        footprint_link.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backward_compatibility_old_format_without_dirs() {
        // Test that old footprint format (without dirs field) parses correctly
        let old_format_toml = r#"
[[links]]
src = "/test/src1"
dest = "/test/dest1"

[[links]]
src = "/test/src2"
dest = "/test/dest2"
"#;

        let footprint: Footprint = toml::from_str(old_format_toml)
            .expect("Should parse old format without dirs field");

        // dirs should default to empty set
        assert!(footprint.dirs.is_empty());

        // links should parse correctly
        assert_eq!(footprint.links.len(), 2);

        // Verify links contain expected paths
        let links: Vec<_> = footprint.links.iter().collect();
        assert!(links.iter().any(|link| link.src.path.as_str() == "/test/src1"));
        assert!(links.iter().any(|link| link.dest.path.as_str() == "/test/dest1"));
    }

    #[test]
    fn test_new_format_with_dirs() {
        // Test that new footprint format (with dirs field) parses correctly
        let new_format_toml = r#"
dirs = ["/test/created_dir1", "/test/created_dir2"]

[[links]]
src = "/test/src1"
dest = "/test/dest1"
"#;

        let footprint: Footprint = toml::from_str(new_format_toml)
            .expect("Should parse new format with dirs field");

        // dirs should contain expected directories
        assert_eq!(footprint.dirs.len(), 2);
        assert!(footprint.dirs.contains(&Utf8PathBuf::from("/test/created_dir1")));
        assert!(footprint.dirs.contains(&Utf8PathBuf::from("/test/created_dir2")));

        // links should parse correctly
        assert_eq!(footprint.links.len(), 1);
    }

    #[test]
    fn test_empty_dirs_serialization() {
        // Test that footprint with empty dirs serializes correctly
        let footprint = Footprint::default();

        let serialized = toml::to_string(&footprint)
            .expect("Should serialize empty footprint");

        // Should include dirs = [] in output
        assert!(serialized.contains("dirs = []"));

        // Should be parseable back
        let parsed: Footprint = toml::from_str(&serialized)
            .expect("Should parse serialized footprint");

        assert!(parsed.dirs.is_empty());
        assert!(parsed.links.is_empty());
    }

    #[test]
    fn test_roundtrip_with_dirs() {
        // Test serialize -> deserialize roundtrip with dirs
        let mut footprint = Footprint::default();
        footprint.dirs.insert(Utf8PathBuf::from("/test/dir"));

        let serialized = toml::to_string(&footprint)
            .expect("Should serialize footprint with dirs");

        let parsed: Footprint = toml::from_str(&serialized)
            .expect("Should parse back serialized footprint");

        assert_eq!(footprint.dirs, parsed.dirs);
        assert_eq!(footprint.links, parsed.links);
    }
}
