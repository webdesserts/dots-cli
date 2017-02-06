use xdg;
use dot_package::DotPackage;
use std::path::{Path, PathBuf};

pub struct Dot {
   pub package: DotPackage
}

impl Dot {
    pub fn new(path: &Path) -> Result<Dot, String> {
        let package = DotPackage::new(path)?;
        Ok(Dot { package: package })
    }
}

pub fn root() -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::new().expect("XDG Initialization Error");
    xdg_dirs.create_config_directory("dots").expect("Error creating config directory")
}

pub fn path<P: AsRef<Path>>(path: P) -> PathBuf {
    root().join(path)
}
