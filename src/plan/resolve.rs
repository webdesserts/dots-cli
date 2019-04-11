use dots::Dot;
use plan::links::{Anchor, AnchorKind, Link};
use std::fmt::Display;
use std::fs::FileType;
use std::path::{Path, PathBuf};
use std::{self, fmt, fs, io, process};

/*================*\
*  Resolved Links  *
\*================*/

pub struct ResolvedLink {
    pub src: ResolvedAnchor,
    pub dest: ResolvedAnchor,
}

pub fn resolve(dot: Dot, link: Link) -> ResolvedLink {
    return ResolvedLink {
        src: resolve_src(link.src, dot.path),
        dest: resolve_dest(link.dest),
    };
}

/*==================*\
*  Resolved Anchors  *
\*==================*/

pub struct ResolvedAnchor {
    pub path: Option<PathBuf>,
    pub original: Anchor,
    pub issues: Vec<ResolveIssue>,
}

impl ResolvedAnchor {
    fn new(original: Anchor) -> Self {
        return ResolvedAnchor {
            path: None,
            original,
            issues: vec![],
        };
    }

    fn kind(&self) -> AnchorKind {
        self.original.kind
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.level() == ResolveIssueLevel::Error)
    }

    fn has_warnings(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.level() == ResolveIssueLevel::Warning)
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    pub fn max_issue_level(&self) -> Option<ResolveIssueLevel> {
        self.issues.iter().map(|issue| issue.level()).max()
    }
}

fn resolve_src<P: AsRef<Path>>(anchor: Anchor, root: P) -> ResolvedAnchor {
    if anchor.kind != AnchorKind::Destination {
        error!("Invalid AnchorKind passed to resolve_dest");
        process::exit(1);
    }

    let root = root.as_ref();
    let mut src = ResolvedAnchor::new(anchor);
    if src.original.path.is_absolute() {
        src.issues.push(ResolveIssue::new(
            &src.original,
            ResolveIssueKind::InvalidPath(String::from("Expected it to be a relative path.")),
        ));

        return src;
    }

    let absolute_path = root.join(&src.original.path);

    match absolute_path.canonicalize() {
        Ok(path) => src.path = Some(path),
        Err(err) => {
            use self::ResolveIssueKind as link;
            use std::io::ErrorKind as io;
            let issue_kind = match err.kind() {
                io::NotFound => link::NotFound,
                io::PermissionDenied => link::PermissionDenied,
                _ => link::IO(err),
            };

            let issue = ResolveIssue::new(&src.original, issue_kind);
            src.issues.push(issue);
        }
    };

    return src;
}

fn resolve_dest(anchor: Anchor) -> ResolvedAnchor {
    if anchor.kind != AnchorKind::Destination {
        error!("Invalid AnchorKind passed to resolve_dest");
        process::exit(1);
    }

    let mut dest = ResolvedAnchor::new(anchor);

    // if the path is relative assume they want you to link to the home directory
    if dest.original.path.is_relative() {
        match std::env::home_dir() {
            Some(home) => {
                let mut relative = dest.original.path.to_str().unwrap();
                // check to see if they already supplied ~/ as the root. If they did, remove it
                if dest.original.path.starts_with("~/") {
                    relative = relative.replace("~/", "");
                };
                // then use join the relative path to the home directory
                dest.path = Some(home.join(relative));
            }
            None => {
                error!("Unable to access Home Directory");
                process::exit(1);
            }
        };
    }

    if let Some(ref path) = dest.path {
        if path.exists() {
            let issue = match path.symlink_metadata() {
                Ok(metadata) => {
                    let file_type = metadata.file_type();
                    ResolveIssue::new(&dest.original, ResolveIssueKind::AlreadyExists(file_type))
                }
                Err(error) => ResolveIssue::io(&dest.original, error),
            };
            dest.issues.push(issue)
        }
    }

    return dest;
}

/*========*\
*  Issues  *
\*========*/

#[derive(Debug)]
pub struct ResolveIssue {
    pub kind: ResolveIssueKind,
    pub anchor: Anchor,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum ResolveIssueLevel {
    Error,
    Warning,
}

#[derive(Debug)]
pub enum ResolveIssueKind {
    AlreadyExists(fs::FileType),
    InvalidPath(String),
    NotFound,
    PermissionDenied,
    IO(io::Error),
    Other(String),
}

impl ResolveIssue {
    fn new(anchor: &Anchor, kind: ResolveIssueKind) -> Self {
        ResolveIssue {
            anchor: anchor.to_owned(),
            kind: kind,
        }
    }

    fn simple(anchor: &Anchor, message: &str) -> Self {
        Self::new(anchor, ResolveIssueKind::Other(message.to_string()))
    }

    fn io(anchor: &Anchor, error: io::Error) -> Self {
        Self::new(anchor, ResolveIssueKind::IO(error))
    }

    fn level(&self) -> ResolveIssueLevel {
        use self::ResolveIssueKind::*;
        use self::ResolveIssueLevel::*;
        match self.kind {
            AlreadyExists(ref file_type) => Warning,
            InvalidPath(ref msg) => Error,
            NotFound => Error,
            PermissionDenied => Error,
            IO(ref err) => Error,
            Other(ref msg) => Error,
        }
    }
}

impl Display for ResolveIssue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ResolveIssueKind::*;
        match self.kind {
            AlreadyExists(ref file_type) => write!(
                f,
                "{} already exists as {}: {} ",
                self.anchor.kind,
                file_type_to_str(file_type),
                self.anchor.path.display()
            ),
            InvalidPath(ref msg) => write!(
                f,
                "{} is not a valid path. {}: {}",
                self.anchor.kind,
                msg,
                self.anchor.path.display()
            ),
            NotFound => write!(
                f,
                "Can't find {}: {} ",
                self.anchor.kind,
                self.anchor.path.display()
            ),
            PermissionDenied => write!(
                f,
                "Permission denied to {}: {} ",
                self.anchor.kind,
                self.anchor.path.display()
            ),
            IO(ref err) => write!(
                f,
                "Error resolving {} {}: {}",
                self.anchor.kind,
                self.anchor.path.display(),
                err
            ),
            Other(ref msg) => write!(
                f,
                "Error resolving {} {}: {}",
                self.anchor.kind,
                self.anchor.path.display(),
                msg
            ),
        }
    }
}

fn file_type_to_str(file_type: &FileType) -> &str {
    if file_type.is_dir() {
        "a directory"
    } else if file_type.is_symlink() {
        "a symbolic link"
    } else if file_type.is_file() {
        "a file"
    } else {
        "an unknown file type"
    }
}
