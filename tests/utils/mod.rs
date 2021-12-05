use camino::{Utf8Path, Utf8PathBuf};
use pretty_assertions::assert_eq;
use std::process;
use std::{fmt::Display, fs, io, process::Output};
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;

pub fn copy_dir<S, D>(source: S, destination: D) -> Result<(), io::Error>
where
    S: AsRef<Utf8Path>,
    D: AsRef<Utf8Path>,
{
    let source = source.as_ref();
    let destination = destination.as_ref();

    for entry in WalkDir::new(source).max_depth(10) {
        let entry = entry?;
        let file_type = entry.file_type();

        let from = Utf8Path::from_path(entry.path()).unwrap();
        let relative = from.strip_prefix(&source).unwrap();
        let to = destination.join(&relative);

        if file_type.is_file() {
            // We're explicitely avoiding using `fs::copy` here as it sends `cargo watch` into an infinite loop
            fs::read(&from)
                .and_then(|file| fs::write(&to, &file))
                .unwrap_or_else(|_| panic!("Failed to copy file from {} to {}", &from, &to));
        }
        if file_type.is_dir() {
            fs::create_dir_all(&to)
                .unwrap_or_else(|_| panic!("Failed to copy dir from {} to {}", &from, &to));
        }
    }
    Ok(())
}

pub fn print_tree<P>(path: P) -> Result<(), failure::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();
    process::Command::new("tree").arg(&path).spawn()?;
    Ok(())
}

pub fn init_git_repo<P>(path: P) -> Result<(), failure::Error>
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref();

    process::Command::new("git")
        .arg("init")
        .current_dir(&path)
        .spawn()?
        .wait()?;

    process::Command::new("git")
        .arg("add")
        .arg("--all")
        .current_dir(&path)
        .spawn()?
        .wait()?;

    process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("\"initial commit\"")
        .current_dir(&path)
        .spawn()?
        .wait()?;

    Ok(())
}

fn current_dir() -> Utf8PathBuf {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    Utf8PathBuf::from_path_buf(current_dir).unwrap()
}

pub struct TestDir {
    tmpdir: TempDir,
}

impl TestDir {
    pub fn new() -> Result<Self, failure::Error> {
        let tmpdir = tempdir()?;
        let test_dir = TestDir { tmpdir };
        Ok(test_dir)
    }

    pub fn path(&self) -> &Utf8Path {
        Utf8Path::from_path(self.tmpdir.path()).unwrap()
    }

    pub fn fixture_root(&self) -> Utf8PathBuf {
        self.path().join("fixtures")
    }

    pub fn dots_root(&self) -> Utf8PathBuf {
        self.path().join("dots")
    }

    pub fn fixture_path(&self, fixture: &Fixture) -> Utf8PathBuf {
        self.fixture_root().join(fixture.name())
    }

    fn fixture_source_path(&self, fixture: &Fixture) -> Utf8PathBuf {
        current_dir().join("fixtures").join(fixture.name())
    }

    pub fn setup_fixture(&self, fixture: &Fixture) -> Result<Utf8PathBuf, failure::Error> {
        let fixture_src = self.fixture_source_path(fixture);
        let fixture_dest = self.fixture_path(fixture);

        copy_dir(&fixture_src, &fixture_dest)
            .unwrap_or_else(|_| panic!("Failed to setup fixture {}", fixture_src));

        Ok(fixture_dest)
    }

    pub fn setup_fixture_as_git_repo(
        &self,
        fixture: &Fixture,
    ) -> Result<Utf8PathBuf, failure::Error> {
        let fixture_path = self.setup_fixture(fixture)?;
        init_git_repo(&fixture_path)?;
        Ok(fixture_path)
    }
}

pub enum Fixture {
    ExampleDot,
}

impl Fixture {
    pub fn name(&self) -> &str {
        match self {
            Self::ExampleDot => "example_dot",
        }
    }
}

impl Display for Fixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

pub trait AssertableOutput {
    fn assert_stderr_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>;
    fn assert_stdout_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>;
    fn assert_success(&self) -> &Self;
    fn assert_fail(&self) -> &Self;
    fn assert_fail_with_signal(&self, signal: i32) -> &Self;
}

impl AssertableOutput for Output {
    fn assert_stdout_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>,
    {
        let expected = expected.as_ref();

        let stdout = self.stdout.clone();
        let stdout_str = std::str::from_utf8(&stdout).unwrap();

        assert_eq!(stdout_str, expected);

        self
    }

    fn assert_stderr_eq<E>(&self, expected: E) -> &Self
    where
        E: AsRef<str>,
    {
        let expected = expected.as_ref();

        let stderr = self.stderr.clone();
        let stderr_str = std::str::from_utf8(&stderr).unwrap();

        assert_eq!(stderr_str, expected);
        self
    }

    fn assert_success(&self) -> &Self {
        assert!(
            self.status.success(),
            "expected command to succeed, but it failed with code {:?}",
            self.status.code()
        );
        self
    }

    fn assert_fail(&self) -> &Self {
        assert!(
            !self.status.success(),
            "expected command to fail, but it succeeded"
        );
        self
    }

    fn assert_fail_with_signal(&self, expected_code: i32) -> &Self {
        let code = self.status.code();
        assert_eq!(
            code,
            Some(expected_code),
            "expected fail signal {:?} but it succeeded with {:?}",
            expected_code,
            code
        );
        self
    }
}
