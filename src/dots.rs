use xdg;
use dot_package::DotPackage;
use std::path::{Path, PathBuf};
use git_utils;
use std::process;
use std::env;

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
    let xdg_dirs = xdg::BaseDirectories::new().expect("XDG Initialization Error");
    xdg_dirs.create_config_directory("dots").expect("Error creating config directory")
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
