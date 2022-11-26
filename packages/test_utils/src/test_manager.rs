use crate::Fixture;
use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::io::Read;
use std::process::Command;
use tempfile::{tempdir, TempDir};
use utils::fs::canonicalize;
use utils::fs::copy_dir;
use utils::fs::empty_git_directory;
use utils::git;

pub struct TestManager {
    tmpdir: TempDir,
}

impl TestManager {
    pub fn new() -> Result<Self> {
        let tmpdir = tempdir()?;
        let test_manager = TestManager { tmpdir };
        Ok(test_manager)
    }

    /** The path of the temporary test directory */
    pub fn home_dir(&self) -> Utf8PathBuf {
        let path = Utf8Path::from_path(self.tmpdir.path()).unwrap();
        canonicalize(path).expect("Unable to canonicalize tmp dir")
    }

    /** The path of the directory all fixtures will initialized at once `TestDir::setup_fixture` is called. */
    pub fn fixtures_dir(&self) -> Utf8PathBuf {
        self.home_dir().join("fixtures")
    }

    /** the path of the directory should be used as the installation root for all dots. */
    pub fn dots_dir(&self) -> Utf8PathBuf {
        self.home_dir().join(".dots")
    }

    /** the path of the directory should be used as the installation root for all dots. */
    pub fn footprint_path(&self) -> Utf8PathBuf {
        self.dots_dir().join("dot-footprint.toml")
    }

    /** The path of that the given fixture will be located at once initialized. */
    pub fn fixture_dir(&self, fixture: &Fixture) -> Utf8PathBuf {
        self.fixtures_dir().join(fixture.name())
    }

    pub fn expected_dot_path(&self, fixture: &Fixture) -> Utf8PathBuf {
        self.dots_dir().join(fixture.name())
    }

    /** Creates a copy of the given fixture in the test directory */
    fn setup_fixture(&self, fixture: &Fixture) -> Result<Utf8PathBuf> {
        let fixture_src = fixture.template_path();
        let fixture_dest = self.fixture_dir(fixture);

        copy_dir(&fixture_src, &fixture_dest)
            .unwrap_or_else(|_| panic!("Failed to setup fixture {}", fixture_src));

        Ok(fixture_dest)
    }

    /** Copies the given fixture to the test directory and then initializes it as a git repository */
    pub fn setup_fixture_as_git_repo(&self, fixture: &Fixture) -> Result<Utf8PathBuf> {
        let fixture_path = self.setup_fixture(fixture)?;
        git::init_repo(&fixture_path)?;

        git::config(&fixture_path, "user.name", "webdesserts")?;
        git::config(&fixture_path, "user.email", "test@webdesserts.com")?;

        git::commit_all(&fixture_path, "initial commit")?;

        Ok(fixture_path)
    }

    /** Creates a copy of the given fixture in the test directory */
    pub fn overwrite_dot(&self, fixture1: &Fixture, fixture2: &Fixture) -> Result<()> {
        let path = self.expected_dot_path(fixture1);
        empty_git_directory(&path)?;
        copy_dir(fixture2.template_path(), &path)?;
        println!("{}{}", &path, fixture2.template_path());
        Ok(())
    }

    pub fn cmd(&self, bin: &'static str) -> Result<Command> {
        let mut cmd = Command::new(&bin);
        cmd.env("HOME", self.home_dir());
        Ok(cmd)
    }

    pub fn read_footprint(&self) -> Result<String> {
        let path = self.footprint_path();
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write_footprint<T: AsRef<str>>(&self, contents: T) -> Result<()> {
        let contents = contents.as_ref();
        fs::write(self.footprint_path(), contents)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! cargo_bin {
    ($bin:expr) => {
        env!(concat!("CARGO_BIN_EXE_", $bin))
    };
}
