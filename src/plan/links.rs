use camino::{Utf8Path, Utf8PathBuf};
use std::fmt;

/*=======*\
*  Links  *
\*=======*/

pub struct Link {
    pub src: Anchor,
    pub dest: Anchor,
}

impl Link {
    pub fn new<P>(src: P, dest: P) -> Link
    where
        P: AsRef<Utf8Path>,
    {
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
    pub path: Utf8PathBuf,
}

impl Anchor {
    pub fn new<P>(path: P, kind: AnchorKind) -> Anchor
    where
        P: AsRef<Utf8Path>,
    {
        Anchor {
            path: path.as_ref().to_owned(),
            kind,
        }
    }

    pub fn new_src<P>(path: P) -> Anchor
    where
        P: AsRef<Utf8Path>,
    {
        Anchor {
            path: path.as_ref().to_owned(),
            kind: AnchorKind::Source,
        }
    }

    pub fn new_dest<P>(path: P) -> Anchor
    where
        P: AsRef<Utf8Path>,
    {
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
