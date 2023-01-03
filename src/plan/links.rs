use camino::{Utf8Path, Utf8PathBuf};
use std::{fmt, fs};

/*=======*\
*  Links  *
\*=======*/

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Link {
    /// The path to the dotfile
    pub src: Anchor,
    /// The the path to the symlink
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

    pub fn exists(&self) -> bool {
        let Ok(path) = fs::read_link(&self.dest.path) else { return false };
        return path == self.src.path;
    }
}

impl fmt::Debug for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.dest.path, self.src.path)
    }
}

/*=========*\
*  Anchors  *
\*=========*/

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Anchor {
    /// Whether the path to a dotfile or a symlink
    pub kind: AnchorKind,
    /// The path to that file
    pub path: Utf8PathBuf,
}

impl Anchor {
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

#[derive(Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
pub enum AnchorKind {
    /// An anchor for dotfile path
    Source,
    /// An anchor for the symlink path
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

#[cfg(test)]
mod tests {
    mod link {
        use crate::plan::links::{AnchorKind, Link};

        #[test]
        fn it_should_create_a_new_link() {
            let link = Link::new("./src.txt", "./dest.txt");
            assert_eq!(link.src.path, "./src.txt");
            assert_eq!(link.src.kind, AnchorKind::Source);
            assert_eq!(link.dest.path, "./dest.txt");
            assert_eq!(link.dest.kind, AnchorKind::Destination);
        }

        #[test]
        fn it_should_display_correctly_when_printed() {
            let link = Link::new("./src.txt", "./dest.txt");
            assert_eq!(format!("{}", link.src.kind), "Source");
            assert_eq!(format!("{}", link.dest.kind), "Destination");
        }
    }
}
