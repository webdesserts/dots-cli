use std::path::{Path, PathBuf};
use std::{process,env,fs};
use dot_package::DotPackage;
use utils;

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
            Ok(_) => utils::git::get_origin(),
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

pub fn add(url: &str, overwrite: bool) {
    info!("Adding {}", url);
    let tmp = self::path(".tmp");

    if tmp.is_dir() {
        warn!("Cleaning left over .tmp directory.\nIt appears another command failed to clean up after itself.");
        utils::fs::clean(&tmp);
    }

    info!("Cloning...");
    utils::git::clone(url, &tmp);

    let dot = match Dot::new(&tmp) {
        Ok(dot) => dot,
        Err(_) => {
            utils::fs::clean(&tmp);
            error!("Repo does not appear to be a Dot");
            process::exit(1);
        }
    };

    let target_dir = self::path(&dot.package.package.name);

    if target_dir.exists() {
        if overwrite {
            warn!("Overwriting pre-existing Dot\n\t{}", target_dir.display());
            utils::fs::clean(&target_dir);
        } else {
            error!("A Dot named {} is already installed. Aborting.", dot.package.package.name);
            error!("pass --overwrite to overwrite the pre-existing Dot");
            utils::fs::clean(&tmp);
            process::exit(1);
        }
    }

    fs::rename(tmp, target_dir).expect("Error renaming repo!");
    info!("Cloning complete!")
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

/*
pub fn link() -> Result<(), Vec<io::Error>> {

    link::Plan::new(find_all());


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
*/
