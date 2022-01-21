use std::collections::HashMap as Map;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use toml;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DotPackageMeta {
    pub name: String,
    pub authors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DotPackage {
    pub package: DotPackageMeta,
    #[serde(default)]
    pub execute: Map<String, Vec<String>>,
    pub link: Map<PathBuf, PathBuf>,
}

impl DotPackage {
    pub fn new(path: &Path) -> Result<DotPackage, &str> {
        let contents = match read_package(path.join("Dot.toml")) {
            Ok(package) => package,
            Err(err) => {
                error!("Error reading Dot.toml:\nin {}\n{}", path.display(), err);
                return Err("Error reading Dot.toml");
            }
        };
        let package = match toml::from_str(&contents) {
            Ok(package) => package,
            Err(err) => {
                error!("Error parsing Dot.toml:\nin {}\n{}", path.display(), err);
                return Err("Error parsing Dot.toml");
            }
        };
        Ok(package)
    }
}

fn read_package<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
