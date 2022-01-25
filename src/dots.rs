use crate::dot_package::DotPackage;
use crate::utils::{self, fs::home};
use camino::{Utf8Path, Utf8PathBuf};
use std::{env, fs, process};
use tempfile::tempdir;

pub struct Dot {
    pub package: DotPackage,
    pub path: Utf8PathBuf,
}

impl Dot {
    pub fn new<P>(path: P) -> Result<Dot, failure::Error>
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

    pub fn origin(&self) -> String {
        match env::set_current_dir(self.path.clone()) {
            Ok(_) => utils::git::get_origin().unwrap_or_else(|err| {
                error!(
                    "error getting origin in git repository {}:\n{}",
                    self.path, err
                );
                process::exit(1);
            }),
            Err(err) => {
                error!("error changing directory to {:?}:\n{}", self.path, err);
                process::exit(1);
            }
        }
    }
}

pub struct Environment {
    root: Utf8PathBuf,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            root: home().join(".dots"),
        }
    }
}

impl Environment {
    pub fn new<P>(root: Option<P>) -> Self
    where
        P: AsRef<Utf8Path>,
    {
        match root {
            Some(path) => Environment {
                root: path.as_ref().to_owned(),
            },
            None => Environment::default(),
        }
    }

    pub fn path<P>(&self, path: P) -> Utf8PathBuf
    where
        P: AsRef<Utf8Path>,
    {
        self.root.join(path)
    }

    pub fn package_path(&self, dot: &Dot) -> Utf8PathBuf {
        self.path(&dot.package.package.name)
    }
}

pub fn add(url: &str, overwrite: bool, env: &Environment) {
    info!("Adding {url}");
    let tmp = tempdir().expect("Unable to create temporary directory");
    let tmp_path = Utf8Path::from_path(tmp.path()).unwrap().join("dot");

    info!("Cloning...");
    utils::git::clone(url, &tmp_path).unwrap_or_else(|error| {
        error!("Unable to clone dot\n{}", error);
        process::exit(1)
    });

    let dot = match Dot::new(&tmp_path) {
        Ok(dot) => dot,
        Err(_) => {
            error!("Repo does not appear to be a Dot");
            process::exit(1);
        }
    };

    let target_dir = env.package_path(&dot);

    if target_dir.exists() {
        if overwrite {
            warn!("Overwriting pre-existing Dot\n{}", target_dir);
            utils::fs::clean(&target_dir);
        } else {
            error!(
                "A Dot named {} is already installed. Aborting.",
                env.package_path(&dot)
            );
            error!("pass --overwrite to overwrite the pre-existing Dot");
            process::exit(1);
        }
    }

    fs::create_dir_all(&target_dir).expect("Creating package root");
    info!("Copying to {}", target_dir);
    match fs::rename(&tmp_path, &target_dir) {
        Ok(_) => info!("Done!"),
        Err(err) => error!("Error adding dot. Copy failed due to the following error:\n  {err}"),
    };
}

pub fn find_all(env: &Environment) -> Vec<Dot> {
    let dir = match env.root.read_dir() {
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

#[cfg(test)]
mod tests {
    mod describe_link_request {
        use std::collections::HashMap;

        use camino::Utf8PathBuf;
        use test_utils::Fixture;

        use crate::dots::Dot;

        #[test]
        fn it_should_contain_the_original_path() -> Result<(), failure::Error> {
            let fixture = Fixture::ExampleDot;
            let dot = Dot::new(fixture.template_path())?;
            assert_eq!(dot.path, fixture.template_path());
            Ok(())
        }

        #[test]
        fn it_should_contain_package_details_from_the_dot_toml() -> Result<(), failure::Error> {
            let fixture = Fixture::ExampleDot;
            let dot = Dot::new(fixture.template_path())?;
            assert_eq!(dot.package.package.name, fixture.name());
            assert_eq!(dot.package.package.authors, vec!["Michael Mullins"]);
            Ok(())
        }

        #[test]
        fn it_should_contain_links_from_the_dot_toml() -> Result<(), failure::Error> {
            let fixture = Fixture::ExampleDot;
            let dot = Dot::new(fixture.template_path())?;
            let expected: HashMap<Utf8PathBuf, Utf8PathBuf> =
                vec![("shell/bashrc", "~/.bashrc"), ("shell/zshrc", "~/.zshrc")]
                    .into_iter()
                    .map(|(key, value)| (Utf8PathBuf::from(key), Utf8PathBuf::from(value)))
                    .collect();

            assert_eq!(dot.package.link, expected);
            Ok(())
        }
    }
}
