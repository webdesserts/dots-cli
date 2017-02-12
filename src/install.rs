use std::{io, error, fmt, process, self};
use utils::links::{Link, Anchor, AnchorKind};
use std::path::{PathBuf};
use dots::{Dot};

#[derive(Debug)]
pub enum PlanError {
    InvalidPath(Anchor),
    NotFound(Anchor),
    AlreadyExists(Anchor),
    PermissionDenied(Anchor),
    Other(io::Error),
    Simple(String)
}

impl PlanError {
    fn new(message: &str) -> PlanError {
        PlanError::Simple(message.to_string())
    }
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlanError::InvalidPath(ref anchor) => write!(f, "{} is not a valid path: {}", anchor.kind, anchor.path.display()),
            PlanError::NotFound(ref anchor) => write!(f, "Can't find {}: {} ", anchor.kind, anchor.path.display()),
            PlanError::AlreadyExists(ref anchor) => write!(f, "{} already exists: {} ", anchor.kind, anchor.path.display()),
            PlanError::PermissionDenied(ref anchor) => write!(f, "Permission denied to {}: {} ", anchor.kind, anchor.path.display()),
            PlanError::Other(ref err) => write!(f, "{}", err),
            PlanError::Simple(ref err) => write!(f, "{}", err)
        }
    }
}

impl error::Error for PlanError {
    fn description(&self) -> &str {
        match *self {
            PlanError::InvalidPath(_) => "Invalid Anchor Path",
            PlanError::NotFound(_) => "Anchor Not Found",
            PlanError::AlreadyExists(_) => "Anchor Path Already Exists",
            PlanError::PermissionDenied(_) => "Permission Denied to Anchor Path",
            PlanError::Other(ref err) => err.description(),
            PlanError::Simple(ref err) => err,
        }
    }
}

pub struct Plan {
    link: Vec<Link>
}

impl Plan {
    pub fn new(dots: Vec<Dot>, force: bool) -> Result<Plan, PlanError>{
        use colored::*;
        let mut has_errors = false;
        let mut suggest_force = false;
        let mut plan = Plan { link: vec![] };

        let checkmark = "✔".green();
        let x = "✖".red();

        for dot in dots {
            let mut dot_errors : Vec<PlanError> = vec![];
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
                    println!("  {} {} => {}", checkmark, org_src.path.display(), org_dest.path.display());
                    plan.link.push(Link::new(src, dest))
                } else {
                    if let Err(err) = resolved_src {
                        let src_path = format!("{}", org_src.path.display());
                        println!("  {} {} => {}", x, src_path.italic().red(), org_dest.path.display());
                        dot_errors.push(err);
                        has_errors = true;
                    }
                    if let Err(err) = resolved_dest {
                        match err {
                            PlanError::AlreadyExists(_) => { suggest_force = true }
                            _ => {}
                        }
                        let dest_path = format!("{}", org_dest.path.display());
                        println!("  {} {} => {}", x, org_src.path.display(), dest_path.italic().red());
                        dot_errors.push(err);
                        has_errors = true;
                    }
                }
            }

            for err in dot_errors {
                println!();
                error!("{}", err)
            }
        }

        println!();
        if suggest_force { info!("{}", "use --force to overwrite existing directories") }
        if has_errors { Err(PlanError::new("Planning failed.")) }
            else { Ok(plan) }
    }

    fn resolve_src (src: &Anchor, root: &PathBuf) -> Result<Anchor, PlanError> {
        let mut resolved = src.clone();
        if resolved.path.is_absolute() {
            return Err(PlanError::InvalidPath(src.clone()))
        }

        resolved.path = root.join(resolved.path);

        resolved.path = match resolved.path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                use std::io::ErrorKind::*;
                match err.kind() {
                    NotFound => { return Err(PlanError::NotFound(src.clone())) },
                    PermissionDenied => { return Err(PlanError::PermissionDenied(src.clone())) },
                    _ => { return Err(PlanError::Other(err)) }
                }
            }
        };

        Ok(resolved)
    }

    fn resolve_dest (dest: &Anchor, force: &bool) -> Result<Anchor, PlanError> {
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
                return Err(PlanError::InvalidPath(dest.clone()))
            }
        }

        if resolved.path.is_dir() && !force {
            return Err(PlanError::AlreadyExists(dest.clone()))
        }

        Ok(resolved)
    }
}

