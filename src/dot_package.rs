use serde_json as json;
use std::path::{Path};
use std::collections::HashMap as Map;
use std::fs;
use std::io::{Read, self};
use std::error::Error;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DotPackage {
    pub name: String,
    pub authors: Vec<String>,
    pub links: Map<String, String>,
}

impl DotPackage {
    pub fn new(path: &Path) -> Result<DotPackage, String> {
        let contents = match read_package(path.join("Dot.json")) {
            Ok(package) => package,
            Err(err) => {
                error!("Error reading Dot.json: {}", err.description());
                return Err(String::from("Error reading Dot.json"))
            }
        };
        let package = match json::from_str(&contents) {
            Ok(package) => package,
            Err(err) => {
                error!("Error parsing Dot.json: {}", err.description());
                return Err(String::from("Error parsing Dot.json"))
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

