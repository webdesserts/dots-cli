use std::fmt;
use std::path::{PathBuf};

#[derive(Clone, Debug)]
pub struct Anchor {
    pub kind: AnchorKind,
    pub path: PathBuf,
}

impl Anchor {
    pub fn new(path: PathBuf, kind: AnchorKind) -> Anchor {
        Anchor { path: path, kind: kind }
    }
}

#[derive(Clone, Debug)]
pub enum AnchorKind {
    Source,
    Destination,
}

impl fmt::Display for AnchorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnchorKind::Source => write!(f, "Source"),
            AnchorKind::Destination => write!(f, "Destination")
        }
    }
}

