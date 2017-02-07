use std::path::{Path, PathBuf};
use std::{process,env};
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
        Ok(val) => val,
        Err(err) => {
            error!("Error while searching for dots:\n{}", err);
            process::exit(1);
        }
    };

    let mut dots = Vec::new();

    for entries in dir {
        match Dot::new(entries.unwrap().path().as_path()).ok() {
            Some(dot) => dots.push(dot),
            None => {}
        }
    }

    dots
}
