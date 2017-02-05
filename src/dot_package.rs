use serde_json as json;
use std::path::Path;
use std::collections::HashMap as Map;
use std::process;
use std::fs;
use std::io::{Read, self};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DotPackage {
    pub name: String,
    pub authors: Vec<String>,
    pub links: Map<String, String>,
}

impl DotPackage {
    pub fn new (path: &Path) -> DotPackage {
        let contents = match read_package(path) {
            Ok(val) => val,
            Err(err) => {
                error!("unable to read Dot.toml at {}", path.to_str().unwrap());
                process::exit(1);
            }
        };

        json::from_str(&contents).unwrap()
    }
}

fn read_package(path: &Path) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

