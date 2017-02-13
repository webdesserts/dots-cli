use std::{io, fs, error, fmt, process, self};
use utils::links::{Anchor, AnchorKind};
use std::path::{PathBuf};
use dots::{Dot};
use colored::*;

#[derive(Debug)]
pub struct ResolveError {
    anchor: Anchor,
    kind: ResolveErrorKind
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


impl fmt::Display for ResolveError {
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

impl error::Error for ResolveError {
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

impl fmt::Display for Action {
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
    fn display_err(&self, error: &ResolveError) -> String {
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

    fn resolve(&self, root: &PathBuf, force: &bool) -> Result<Action, ResolveError> {
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
                return Err(ResolveError::new(dest, ResolveErrorKind::InvalidPath))
            }
        }

        if resolved.path.is_dir() && !force {
            return Err(ResolveError::new(dest, ResolveErrorKind::AlreadyExists))
        }

        Ok(resolved)
    }
}

#[derive(Debug)]
pub struct PlanError {
    msg: String,
}

impl PlanError {
    fn new(msg: &str) -> PlanError {
        PlanError { msg: msg.to_string() }
    }
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plan Error: {}", self.msg)
    }
}

impl error::Error for PlanError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

pub struct Plan {
    actions: Vec<Action>,
}

impl Plan {
    pub fn new(dots: Vec<Dot>, force: bool) -> Result<Plan, PlanError>{
        use colored::*;

        let mut suggest_force = false;
        let mut plan = Plan { actions: vec![] };
        let mut errors = vec![];

        for dot in dots {
            let title = format!("[{}]", &dot.package.package.name);
            println!("\n{}", title.bold());

            for (src, dest) in dot.package.link {
                let requested_action = Action::Link {
                    src: Anchor::new(src, AnchorKind::Source),
                    dest: Anchor::new(dest, AnchorKind::Destination)
                };

                match requested_action.resolve(&dot.path, &force) {
                    Ok(action) => {
                        println!("  {}", requested_action);
                        plan.actions.push(action)
                    }
                    Err(err) => {
                        match err.kind {
                            ResolveErrorKind::AlreadyExists => suggest_force = true,
                            _ => {}
                        }

                        println!("  {}", requested_action.display_err(&err));
                        errors.push(err)
                    }
                }
            }
        }

        let has_errors = !errors.is_empty();

        if has_errors { println!() }
        for err in errors {
            error!("{}", err)
        }

        println!();
        if suggest_force { info!("{}", "use --force to overwrite existing directories") }
        if has_errors {
            Err(PlanError::new("Planning failed."))
        } else {
            Ok(plan)
        }
    }

    pub fn execute(&self, force: bool) -> io::Result<()> {
        for action in &self.actions {
            match *action {
                Action::Link { ref src, ref dest } => {
                    if dest.path.exists() {
                        let file_type = dest.path.metadata()?.file_type();
                        if file_type.is_symlink() || file_type.is_file() {
                            fs::remove_file(&dest.path)?;
                        } else if file_type.is_dir() {
                            if force {
                                fs::remove_dir_all(&dest.path)?;
                            } else {
                                return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Destination already Exists!"));
                            }
                        };
                    };

                    if let Some(parent) = dest.path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    std::os::unix::fs::symlink(&src.path, &dest.path)?;
                }
            }
        }
        Ok(())
    }
}

