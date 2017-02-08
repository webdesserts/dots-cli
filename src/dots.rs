use std::path::{Path, PathBuf};
use std::collections::HashMap as Map;
use std::{process,env,io,fs,os};
use dot_package::DotPackage;
use git_utils;

pub struct Dot {
    pub package: DotPackage,
    pub path: PathBuf
}

impl Dot {
    pub fn new(path: &Path) -> Result<Dot, String> {
        let package = DotPackage::new(path)?;
        Ok(Dot { package: package, path: path.to_path_buf() })
    }

    pub fn origin(&self) -> Option<String> {
        match env::set_current_dir(self.path.clone()) {
            Ok(_) => git_utils::get_origin(),
            Err(err) => {
                error!("error changing directory to {:?}:\n{}", self.path, err);
                process::exit(1);
            }
        }
    }
}

pub fn root() -> PathBuf {
    match env::home_dir() {
        Some(home) => home.join(".dots"),
        None => {
            error!("Unable to access home directory");
            process::exit(1)
        }
    }
}

pub fn path<P: AsRef<Path>>(path: P) -> PathBuf {
    root().join(path)
}

pub fn find_all() -> Vec<Dot> {
    let dir = match root().read_dir() {
        Ok(read_dir) => read_dir,
        Err(err) => {
            use std::io::ErrorKind as Kind;
            match err.kind() {
                Kind::NotFound => { return vec![] },
                Kind::PermissionDenied => {
                    error!("Unable access dots directory:\n{}", err);
                    process::exit(1);
                },
                _ => {
                    error!("Error while accessing dots directory:\n{}", err);
                    process::exit(1);
                }
            }
        }
    };

    let mut dots = Vec::new();

    for entry in dir {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(_) => continue,
        };
        match Dot::new(path.as_path()) {
            Ok(dot) => dots.push(dot),
            Err(_) => {}
        }
    }

    dots
}

type LinkPlan = Map<PathBuf, PathBuf>;

pub fn link() -> Result<(), Vec<io::Error>> {
    let mut plan_errors = vec![];
    let mut absolute_links : LinkPlan = Map::new();
    let dots = find_all();
    for dot in dots {
        for (src, dest) in dot.package.link {
            match dot.path.join(src).canonicalize() {
                Ok(resolved) => { absolute_links.insert(resolved, dot.path.join(dest)); },
                Err(err) => { plan_errors.push(err); }
            };
        }
    }

    if !plan_errors.is_empty() {
        return Err(plan_errors)
    }

    for (src, dest) in absolute_links {
        let parent = match dest.parent() {
            Some(val) => val,
            None => {
                return Err(vec![io::Error::new(io::ErrorKind::InvalidInput, "Cannot symlink to root")]);
            }
        };

        match fs::create_dir_all(parent) {
            Err(err) => { return Err(vec![err]); }
            _ => {}
        }

        match os::unix::fs::symlink(src, dest.clone()) {
            Err(err) => { return Err(vec![err]) }
            _ => {}
        }
    }

    Ok(())
}
