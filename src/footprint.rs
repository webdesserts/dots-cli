use std::collections::BTreeSet;

use crate::plan::links::{Anchor, Link};
use camino::Utf8PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Footprint {
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

impl Into<Link> for FootprintLink {
    fn into(self) -> Link {
        Link {
            src: Anchor::new_src(self.src),
            dest: Anchor::new_dest(self.dest),
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
