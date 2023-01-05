use crate::plan::links::{Anchor, AnchorKind, Link};
use crate::utils::fs::{canonicalize, home};
use camino::{Utf8Path, Utf8PathBuf};
use std::fmt::Display;
use std::fs::FileType;
use std::path::PathBuf;
use std::{fmt, fs, io, process};

mod styles {
    use utils::stylize::Style;

    pub const OK: Style = Style::new().green();
    pub const ERROR: Style = Style::new().red();
    pub const WARN: Style = Style::new().yellow();

    pub const WARN_PATH: Style = WARN.underlined();
    pub const ERROR_PATH: Style = ERROR.italic();
}

pub fn resolve<P>(root: P, link: Link) -> ResolvedLink
where
    P: AsRef<Utf8Path>,
{
    let src = resolve_src(link.src, &root);
    let dest = resolve_dest(link.dest, &src);
    ResolvedLink { src, dest }
}

fn resolve_src<P>(anchor: Anchor, root: P) -> ResolvedAnchor
where
    P: AsRef<Utf8Path>,
{
    if anchor.kind != AnchorKind::Source {
        error!("Invalid AnchorKind passed to resolve_src");
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

    match canonicalize(absolute_path) {
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

    src
}

fn resolve_dest(anchor: Anchor, src: &ResolvedAnchor) -> ResolvedAnchor {
    if anchor.kind != AnchorKind::Destination {
        error!("Invalid AnchorKind passed to resolve_dest");
        process::exit(1);
    }

    let mut dest = ResolvedAnchor::new(anchor);

    // if the path is relative assume they want you to link to the home directory
    if dest.original.path.is_relative() {
        let home = home();
        let mut relative = dest.original.path.to_string();
        // check to see if they already supplied ~/ as the root. If they did, remove it
        if dest.original.path.starts_with("~/") {
            relative = relative.replace("~/", "");
        };
        // then use join the relative path to the home directory
        dest.path = Some(home.join(relative));
    }

    if let Some(ref path) = dest.path {
        if path.exists() {
            // We use symlink_metadata here so that we don't follow any symlinks that we run into
            let issue = match path.symlink_metadata() {
                Ok(metadata) => {
                    let file_type = metadata.file_type();
                    if file_type.is_symlink() {
                        let src_path: Option<PathBuf> = src.path.as_ref().map(|path| path.into());
                        match path.read_link() {
                            Ok(linked_path) => {
                                if Some(linked_path) == src_path {
                                    None
                                } else {
                                    Some(ResolveIssue::new(
                                        &dest.original,
                                        ResolveIssueKind::AlreadyExists(file_type),
                                    ))
                                }
                            }
                            Err(_) => None,
                        }
                    } else {
                        Some(ResolveIssue::new(
                            &dest.original,
                            ResolveIssueKind::AlreadyExists(file_type),
                        ))
                    }
                }
                Err(error) => Some(ResolveIssue::io(&dest.original, error)),
            };
            if let Some(issue) = issue {
                dest.issues.push(issue)
            }
        }
    }

    dest
}

/*================*\
*  Resolved Links  *
\*================*/

/// A Link where both the symlink and dotfile path have been resolved and checked for issues
#[derive(PartialEq, Eq)]
pub struct ResolvedLink {
    /// The resolved anchor for the dotfile
    pub src: ResolvedAnchor,
    /// The resolved anchor for the symlink
    pub dest: ResolvedAnchor,
}

impl ResolvedLink {
    pub fn issues(&self) -> Vec<&ResolveIssue> {
        let src_issues = self.src.issues.iter();
        let dest_issues = self.dest.issues.iter();

        src_issues.chain(dest_issues).collect()
    }

    pub fn has_errors(&self) -> bool {
        self.src.has_errors() | self.dest.has_errors()
    }

    pub fn has_warnings(&self) -> bool {
        self.src.has_errors() | self.dest.has_warnings()
    }
    /// Returns a simplified link if all paths are valid
    pub fn as_link(&self) -> Option<Link> {
        let Some(src) = &self.src.path else { return None };
        let Some(dest) = &self.dest.path else { return None };
        Some(Link {
            src: Anchor::new_src(src),
            dest: Anchor::new_dest(dest),
        })
    }
}

impl Display for ResolvedLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::plan::resolve::ResolveIssueLevel::*;
        let src = &self.src;
        let dest = &self.dest;

        let statusmark = if src.has_errors() | dest.has_errors() {
            styles::ERROR.apply("✖")
        } else {
            styles::OK.apply("✔")
        };

        let mut src_path = src.original.path.to_string();
        let is_directory = match &src.path {
            Some(path) => path.is_dir(),
            _ => false,
        };

        if is_directory {
            src_path += "/"
        }

        let src_msg = match src.max_issue_level() {
            Some(Error) => styles::ERROR_PATH.apply(src_path).to_string(),
            Some(Warning) => styles::WARN_PATH.apply(src_path).to_string(),
            None => src_path,
        };

        let dest_path = dest.original.path.to_string();
        let dest_msg = match dest.max_issue_level() {
            Some(Error) => styles::ERROR_PATH.apply(dest_path).to_string(),
            Some(Warning) => styles::WARN_PATH.apply(dest_path).to_string(),
            None => dest_path,
        };

        write!(f, "{} {} => {}", statusmark, src_msg, dest_msg)
    }
}

/*==================*\
*  Resolved Anchors  *
\*==================*/

/// A ResolvedAnchor is an Anchor whos path has been cannonicalized and checked for potential issues.
/// Any issues that are found are collected for reporting back to the user.
#[derive(PartialEq, Eq)]
pub struct ResolvedAnchor {
    /// Resolved path. If the path is not a valid FS path it will be `None`
    pub path: Option<Utf8PathBuf>,
    /// The original unresolved anchor
    pub original: Anchor,
    /// Issues that were encountered while resolving the path
    pub issues: Vec<ResolveIssue>,
}

impl ResolvedAnchor {
    fn new(original: Anchor) -> Self {
        ResolvedAnchor {
            path: None,
            original,
            issues: vec![],
        }
    }

    pub fn kind(&self) -> &AnchorKind {
        &self.original.kind
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.level() == ResolveIssueLevel::Error)
    }

    pub fn has_warnings(&self) -> bool {
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

    pub fn mark_as_duplicate(&mut self) {
        self.issues.push(ResolveIssue::new(
            &self.original,
            ResolveIssueKind::Conflict,
        ))
    }
}

/*========*\
*  Issues  *
\*========*/

/**
 * ## Errors & Warnings
 * There are two types of errors:
 * Errors resolving the source of a link
 * Errors resolving the destination of a link
 *
 * Warnings – issues that we can resolve, but should require user confirmation.
 * The install should still be resumable once a warning is confirmed.
 * (e.g Are you sure you want to overwrite this directory?)
 *
 * Errors – issues that we can't control or issues that need to be resolved by
 * the user. Errors should stop the install in its tracks.
 */

#[derive(Debug, Eq, PartialEq)]
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
    Conflict,
    AlreadyExists(fs::FileType),
    InvalidPath(String),
    NotFound,
    PermissionDenied,
    IO(io::Error),
}

impl Eq for ResolveIssueKind {}

impl PartialEq for ResolveIssueKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AlreadyExists(a), Self::AlreadyExists(b)) => a == b,
            (Self::InvalidPath(a), Self::InvalidPath(b)) => a == b,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl ResolveIssue {
    fn new(anchor: &Anchor, kind: ResolveIssueKind) -> Self {
        ResolveIssue {
            anchor: anchor.to_owned(),
            kind,
        }
    }

    fn io(anchor: &Anchor, error: io::Error) -> Self {
        Self::new(anchor, ResolveIssueKind::IO(error))
    }

    pub fn level(&self) -> ResolveIssueLevel {
        use self::ResolveIssueKind::*;
        use self::ResolveIssueLevel::*;
        match self.kind {
            Conflict => Error,
            AlreadyExists(_) => Warning,
            InvalidPath(_) => Error,
            NotFound => Error,
            PermissionDenied => Error,
            IO(_) => Error,
        }
    }
}

impl Display for ResolveIssue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ResolveIssueKind::*;
        match self.kind {
            Conflict => write!(
                f,
                "Multiple dots link to the following {}: {}",
                self.anchor.kind.to_string().to_lowercase(),
                self.anchor.path
            ),
            AlreadyExists(ref file_type) => {
                let kind = &self.anchor.kind;
                let file_type_str = file_type_to_str(file_type);
                let path = &self.anchor.path;
                if self.anchor.kind == AnchorKind::Destination && file_type.is_symlink() {
                    write!(
                        f,
                        "{kind} already exists as {file_type_str} to another file: {path}",
                    )
                } else {
                    write!(f, "{kind} already exists as {file_type_str}: {path}",)
                }
            }
            InvalidPath(ref msg) => write!(
                f,
                "{} is not a valid path. {}: {}",
                self.anchor.kind, msg, self.anchor.path
            ),
            NotFound => write!(f, "Can't find {}: {}", self.anchor.kind, self.anchor.path),
            PermissionDenied => write!(
                f,
                "Permission denied to {}: {}",
                self.anchor.kind, self.anchor.path
            ),
            IO(ref err) => write!(
                f,
                "Error resolving {} {}: {}",
                self.anchor.kind, self.anchor.path, err
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
