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
