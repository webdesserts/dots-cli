use camino::{Utf8Path, Utf8PathBuf};
use dot_package::DotPackage;
use std::{env, fs, process};
use utils::{self, fs::home};

pub struct Dot {
    pub package: DotPackage,
    pub path: Utf8PathBuf,
}

impl Dot {
    pub fn new<P>(path: P) -> Result<Dot, String>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        let package = DotPackage::new(path)?;
        Ok(Dot {
            package,
            path: path.to_owned(),
        })
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

pub fn root() -> Utf8PathBuf {
    home().join(".dots")
}

pub fn path<P>(path: P) -> Utf8PathBuf
where
    P: AsRef<Utf8Path>,
{
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
            warn!("Overwriting pre-existing Dot\n\t{}", target_dir);
            utils::fs::clean(&target_dir);
        } else {
            error!(
                "A Dot named {} is already installed. Aborting.",
                dot.package.package.name
            );
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
                Kind::NotFound => return vec![],
                Kind::PermissionDenied => {
                    error!("Unable access dots directory:\n{}", err);
                    process::exit(1);
                }
                _ => {
                    error!("Error while accessing dots directory:\n{}", err);
                    process::exit(1);
                }
            }
        }
    };

    let mut dots = Vec::new();

    for entry in dir {
        let maybe_path = entry.map(|entry| entry.path()).ok();
        let maybe_utf8_path = maybe_path.and_then(|p| Utf8PathBuf::from_path_buf(p).ok());

        let path = match maybe_utf8_path {
            Some(path) => path,
            None => {
                continue;
            }
        };

        match Dot::new(path) {
            Ok(dot) => dots.push(dot),
            Err(_) => {}
        }
    }

    dots
}
