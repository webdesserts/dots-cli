use std::{io, fmt, error, process, self};
use std::path::{PathBuf};
use dots::{Dot};

#[derive(Clone, Debug)]
struct Link {
    src: Anchor,
    dest: Anchor
}

impl Link {
    fn new(src: Anchor, dest: Anchor) -> Link {
        Link { src: src, dest: dest }
    }
}

#[derive(Clone, Debug)]
pub struct Anchor {
    kind: AnchorKind,
    path: PathBuf,
}

impl Anchor {
    fn new(path: PathBuf, kind: AnchorKind) -> Anchor {
        Anchor { path: path, kind: kind }
    }
}

#[derive(Clone, Debug)]
enum AnchorKind {
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

#[derive(Debug)]
pub enum Error {
    InvalidPath(Anchor),
    NotFound(Anchor),
    AlreadyExists(Anchor),
    PermissionDenied(Anchor),
    Other(io::Error),
    Simple(String)
}

impl Error {
    fn new(message: &str) -> Error {
        Error::Simple(message.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidPath(ref anchor) => write!(f, "{} is not a valid path: {}", anchor.kind, anchor.path.display()),
            Error::NotFound(ref anchor) => write!(f, "Can't find {}: {} ", anchor.kind, anchor.path.display()),
            Error::AlreadyExists(ref anchor) => write!(f, "{} already exists: {} ", anchor.kind, anchor.path.display()),
            Error::PermissionDenied(ref anchor) => write!(f, "Permission denied to {}: {} ", anchor.kind, anchor.path.display()),
            Error::Other(ref err) => write!(f, "{}", err),
            Error::Simple(ref err) => write!(f, "{}", err)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidPath(_) => "Invalid Anchor Path",
            Error::NotFound(_) => "Anchor Not Found",
            Error::AlreadyExists(_) => "Anchor Path Already Exists",
            Error::PermissionDenied(_) => "Permission Denied to Anchor Path",
            Error::Other(ref err) => err.description(),
            Error::Simple(ref err) => err,
        }
    }
}

pub struct Plan {
    link: Vec<Link>
}

impl Plan {
    pub fn new(dots: Vec<Dot>, force: bool) -> Result<Plan, Error>{
        let mut has_errors = false;
        let mut suggest_force = false;
        let mut plan = Plan { link: vec![] };

        for dot in dots {
            use colored::*;
            let title = format!("[{}]", &dot.package.name);
            println!("\n{}", title.bold());
            for (src, dest) in dot.package.link {
                let org_src = Anchor::new(src, AnchorKind::Source);
                let org_dest = Anchor::new(dest, AnchorKind::Destination);
                let resolved_src = Self::resolve_src(&org_src, &dot.path);
                let resolved_dest = Self::resolve_dest(&org_dest, &force);

                if resolved_src.is_ok() && resolved_dest.is_ok() {
                    let src = resolved_src.unwrap();
                    let dest = resolved_dest.unwrap();
                    info!("{} => {}", org_src.path.display(), org_dest.path.display());
                    //info!("{} => {}", src.path.display(), dest.path.display());
                    plan.link.push(Link::new(src, dest))
                } else {
                    if let Err(err) = resolved_src {
                        let src_path = format!("{}", org_src.path.display());
                        error!("{} => {}", src_path.italic().red(), org_dest.path.display());
                        error!("{}", err); has_errors = true
                    }
                    if let Err(err) = resolved_dest {
                        match err {
                            Error::AlreadyExists(_) => { suggest_force = true }
                            _ => {}
                        }
                        let dest_path = format!("{}", org_dest.path.display());
                        error!("{} => {}", org_src.path.display(), dest_path.italic().red());
                        error!("{}", err); has_errors = true
                    }
                }
            }
        }

        println!();
        if suggest_force { info!("{}", "use --force to overwrite existing directories") }
        if has_errors { Err(Error::new("Planning failed.")) }
        else { Ok(plan) }
    }

    fn resolve_src (src: &Anchor, root: &PathBuf) -> Result<Anchor, Error> {
        let mut resolved = src.clone();
        if resolved.path.is_absolute() {
            return Err(Error::InvalidPath(src.clone()))
        }

        resolved.path = root.join(resolved.path);

        resolved.path = match resolved.path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                use std::io::ErrorKind::*;
                match err.kind() {
                    NotFound => { return Err(Error::NotFound(src.clone())) },
                    PermissionDenied => { return Err(Error::PermissionDenied(src.clone())) },
                    _ => { return Err(Error::Other(err)) }
                }
            }
        };

        Ok(resolved)
    }

    fn resolve_dest (dest: &Anchor, force: &bool) -> Result<Anchor, Error> {
        let mut resolved = dest.clone();

        if resolved.path.is_relative() {
            if resolved.path.starts_with("~") {
                match std::env::home_dir() {
                    Some(home) => {
                        let relative = resolved.path.components().skip(1).fold(String::new(), |old, comp| {
                            if old.len() > 0 { old + "/" + comp.as_os_str().to_str().unwrap() }
                            else { old + comp.as_os_str().to_str().unwrap() }

                        });
                        resolved.path = home.join(relative);
                    },
                    None => {
                        error!("Unable to access Home Directory");
                        process::exit(1);
                    }
                };
            } else {
                return Err(Error::InvalidPath(dest.clone()))
            }
        }

        if resolved.path.is_dir() && !force {
            return Err(Error::AlreadyExists(dest.clone()))
        }

        Ok(resolved)
    }
}

