use std::fmt;
use std::path::{Path, PathBuf};

/*=======*\
*  Links  *
\*=======*/

pub struct Link {
    pub src: Anchor,
    pub dest: Anchor,
}

impl Link {
    pub fn new<P: AsRef<Path>>(src: P, dest: P) -> Link {
        Link {
            src: Anchor::new_src(src),
            dest: Anchor::new_dest(dest),
        }
    }
}

/*=========*\
*  Anchors  *
\*=========*/

#[derive(Clone, Debug)]
pub struct Anchor {
    pub kind: AnchorKind,
    pub path: PathBuf,
}

impl Anchor {
    pub fn new<P: AsRef<Path>>(path: P, kind: AnchorKind) -> Anchor {
        Anchor {
            path: path.as_ref().to_owned(),
            kind: kind,
        }
    }

    pub fn new_src<P: AsRef<Path>>(path: P) -> Anchor {
        Anchor {
            path: path.as_ref().to_owned(),
            kind: AnchorKind::Source,
        }
    }

    pub fn new_dest<P: AsRef<Path>>(path: P) -> Anchor {
        Anchor {
            path: path.as_ref().to_owned(),
            kind: AnchorKind::Destination,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum AnchorKind {
    Source,
    Destination,
}

impl fmt::Display for AnchorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnchorKind::Source => write!(f, "Source"),
            AnchorKind::Destination => write!(f, "Destination"),
        }
    }
}
