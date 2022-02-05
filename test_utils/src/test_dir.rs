use crate::Fixture;
use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use tempfile::{tempdir, TempDir};
use utils::fs::copy_dir;
use utils::git;

pub struct TestDir {
    tmpdir: TempDir,
}

impl TestDir {
    pub fn new() -> Result<Self> {
        let tmpdir = tempdir()?;
        let test_dir = TestDir { tmpdir };
        Ok(test_dir)
    }

    /** The path of the temporary test directory */
    pub fn path(&self) -> &Utf8Path {
        Utf8Path::from_path(self.tmpdir.path()).unwrap()
    }

    /** The path of the directory all fixtures will initialized at once `TestDir::setup_fixture` is called. */
    pub fn fixtures_root(&self) -> Utf8PathBuf {
        self.path().join("fixtures")
    }

    /** the path of the directory should be used as the installation root for all dots. */
    pub fn dots_root(&self) -> Utf8PathBuf {
        self.path().join("dots")
    }

    /** The path of that the given fixture will be located at once initialized. */
    pub fn fixture_path(&self, fixture: &Fixture) -> Utf8PathBuf {
        self.fixtures_root().join(fixture.name())
    }

    /** Creates a copy of the given fixture in the test directory */
    fn copy_fixture(&self, fixture: &Fixture) -> Result<Utf8PathBuf> {
        let fixture_src = fixture.template_path();
        let fixture_dest = self.fixture_path(fixture);

        copy_dir(&fixture_src, &fixture_dest)
            .expect(format!("Failed to setup fixture {}", fixture_src).as_str());

        Ok(fixture_dest)
    }

    /** Copies the given fixture to the test directory and then initializes it as a git repository */
    pub fn setup_fixture_as_git_repo(&self, fixture: &Fixture) -> Result<Utf8PathBuf> {
        let fixture_path = self.copy_fixture(fixture)?;
        git::init_repo(&fixture_path)?;

        git::config(&fixture_path, "user.name", "webdesserts")?;
        git::config(&fixture_path, "user.email", "test@webdesserts.com")?;

        git::commit_all(&fixture_path, "initial commit")?;

        Ok(fixture_path)
    }
}
