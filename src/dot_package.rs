use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::io::{self, Read};
use std::{collections::HashMap as Map, path::Path};
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
    pub link: Map<Utf8PathBuf, Utf8PathBuf>,
}

impl DotPackage {
    pub fn new<P>(path: P) -> Result<DotPackage, &'static str>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        let contents = match read_package(path.join("Dot.toml")) {
            Ok(package) => package,
            Err(err) => {
                error!("Error reading Dot.toml:\nin {}\n{}", path, err);
                return Err("Error reading Dot.toml");
            }
        };
        let package: DotPackage = match toml::from_str(&contents) {
            Ok(package) => package,
            Err(err) => {
                error!("Error parsing Dot.toml:\nin {}\n{}", path, err);
                return Err("Error parsing Dot.toml");
            }
        };
        Ok(package)
    }
}

fn read_package<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
