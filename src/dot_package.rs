use camino::{Utf8Path, Utf8PathBuf};
use failure::format_err;
use std::fs;
use std::io::{self, Read};
use std::{collections::HashMap, path::Path};


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
    pub execute: HashMap<String, Vec<String>>,
    pub link: HashMap<Utf8PathBuf, Utf8PathBuf>,
}

impl DotPackage {
    pub fn new<P>(path: P) -> Result<DotPackage, failure::Error>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        let contents = match read_package(path.join("Dot.toml")) {
            Ok(contents) => contents,
            Err(err) => {
                error!("Error reading Dot.toml:\nin {}\n{}", path, err);
                return Err(format_err!("Error reading Dot.toml"));
            }
        };

        let package = match parse_package(&contents) {
            Ok(package) => package,
            Err(err) => {
                error!("Error parsing Dot.toml:\nin {}\n{}", path, err);
                return Err(format_err!("Error reading Dot.toml"));
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

fn parse_package<S>(contents: S) -> Result<DotPackage, toml::de::Error>
where
    S: AsRef<str>,
{
    toml::from_str(contents.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    mod dot_package {
        use camino::Utf8PathBuf;

        use super::parse_package;
        type TestResult = Result<(), failure::Error>;

        const EXAMPLE_PACKAGE: &str = r#"
        [package]
        name = "my_package_name"
        authors = [ "Michael Mullins" ]

        [link]
        "shell/bashrc" = "~/.bashrc"
        "shell/gitconfig" = "~/.gitconfig"
        "#;

        const BAD_PACKAGE: &str = r#"
        [package]
        name = "my_package_name"
        authors = { "Michael Mullins" }
        "#;

        #[test]
        fn it_should_parse_the_package() -> TestResult {
            let result = parse_package(EXAMPLE_PACKAGE)?;
            assert_eq!(result.package.name, "my_package_name");
            assert_eq!(result.package.authors[0], "Michael Mullins");
            Ok(())
        }

        #[test]
        fn it_should_parse_the_list_of_links() -> TestResult {
            let result = parse_package(EXAMPLE_PACKAGE)?;
            let key = Utf8PathBuf::from("shell/bashrc");
            let value = result.link.get(&key);
            let expected = Utf8PathBuf::from("~/.bashrc");
            assert_eq!(value, Some(&expected));
            Ok(())
        }

        #[test]
        #[should_panic(expected = "kind: Wanted")]
        fn it_should_throw_an_error_when_parse_errors_fail() {
            parse_package(BAD_PACKAGE).unwrap();
        }
    }
}
