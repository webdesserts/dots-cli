use std::{io, fmt, process};
use std::fmt::{Display};
use std::error::{Error};
use std::path::{PathBuf};
use colored::*;
use plan::links::{Anchor, AnchorKind};
use dirs::{home_dir};

#[derive(Debug)]
pub struct ResolveError {
    pub anchor: Anchor,
    pub kind: ResolveErrorKind
}

impl ResolveError {
    fn new(anchor: &Anchor, kind: ResolveErrorKind) -> ResolveError {
        ResolveError { anchor: anchor.clone(), kind: kind }
    }

    fn simple(anchor: &Anchor, message: &str) -> ResolveError {
        Self::new(anchor, ResolveErrorKind::Simple(message.to_string()))
    }
}

#[derive(Debug)]
pub enum ResolveErrorKind {
    InvalidPath,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    Other(io::Error),
    Simple(String)
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ResolveErrorKind::*;
        match self.kind {
            InvalidPath => write!(f, "{} is not a valid path: {}", self.anchor.kind, self.anchor.path.display()),
            NotFound => write!(f, "Can't find {}: {} ", self.anchor.kind, self.anchor.path.display()),
            AlreadyExists => write!(f, "{} already exists: {} ", self.anchor.kind, self.anchor.path.display()),
            PermissionDenied => write!(f, "Permission denied to {}: {} ", self.anchor.kind, self.anchor.path.display()),
            Other(ref err) => write!(f, "Error resolving {} {}: {}", self.anchor.kind, self.anchor.path.display(), err),
            Simple(ref msg) => write!(f, "Error resolving {} {}: {}", self.anchor.kind, self.anchor.path.display(), msg)
        }
    }
}

impl Error for ResolveError {
    fn description(&self) -> &str {
        use self::ResolveErrorKind::*;
        match self.kind {
            InvalidPath => "Invalid Anchor Path",
            NotFound => "Anchor Not Found",
            AlreadyExists => "Anchor Path Already Exists",
            PermissionDenied => "Permission Denied to Anchor Path",
            Other(ref err) => err.description(),
            Simple(ref msg) => msg,
        }
    }
}

pub enum Action {
    Link { src: Anchor, dest: Anchor },
    // Unlink { src: Anchor, dest: Anchor }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checkmark = "✔".green();
        match *self {
            Action::Link{ ref src, ref dest } => {
                write!(f, "{} {} => {}", checkmark, src.path.display(), dest.path.display())
            }
        }
    }
}

impl Action {
    pub fn display_err(&self, error: &ResolveError) -> String {
        let cross = "✖".red();

        match *self {
            Action::Link { ref src, ref dest } => {
                match error.anchor.kind {
                    AnchorKind::Source => {
                        let src_str = format!("{}", src.path.display());
                        format!("{} {} => {}", cross, src_str.red().italic(), dest.path.display())
                    },
                    AnchorKind::Destination => {
                        let dest_str = format!("{}", dest.path.display());
                        format!("{} {} => {}", cross, src.path.display(), dest_str.red().italic())
                    },
                }
            }
        }
    }

    pub fn resolve(&self, root: &PathBuf, force: &bool) -> Result<Action, ResolveError> {
        match * self {
            Action::Link { ref src, ref dest } => {
                let resolved_src = Self::resolve_src(src, root)?;
                let resolved_dest = Self::resolve_dest(dest, force)?;
                Ok(Action::Link { src: resolved_src, dest: resolved_dest })
            }
        }
    }

    fn resolve_src (src: &Anchor, root: &PathBuf) -> Result<Anchor, ResolveError> {
        let mut resolved = src.clone();
        if resolved.path.is_absolute() {
            return Err(ResolveError::new(src, ResolveErrorKind::InvalidPath))
        }

        resolved.path = root.join(resolved.path);

        resolved.path = match resolved.path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                use std::io::ErrorKind::*;
                match err.kind() {
                    NotFound => { return Err(ResolveError::new(src, ResolveErrorKind::NotFound)) },
                    PermissionDenied => { return Err(ResolveError::new(src, ResolveErrorKind::PermissionDenied)) },
                    _ => { return Err(ResolveError::new(src, ResolveErrorKind::Other(err))) }
                }
            }
        };

        Ok(resolved)
    }

    fn resolve_dest (dest: &Anchor, force: &bool) -> Result<Anchor, ResolveError> {
        let mut resolved = dest.clone();

        if resolved.path.is_relative() {
            if resolved.path.starts_with("~/") {
                match home_dir() {
                    Some(home) => {
                        let relative = resolved.path.to_str().unwrap().replace("~/", "");
                        resolved.path = home.join(relative);
                    },
                    None => {
                        error!("Unable to access Home Directory");
                        process::exit(1);
                    }
                };
            } else {
                return Err(ResolveError::new(dest, ResolveErrorKind::InvalidPath))
            }
        }

        if resolved.path.is_dir() && !force {
            return Err(ResolveError::new(dest, ResolveErrorKind::AlreadyExists))
        }

        Ok(resolved)
    }
}
