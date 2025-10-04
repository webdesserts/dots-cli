use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::{fs, io, os::unix};

use crate::{dots::Environment, footprint::Footprint, plan::links::Link};

pub struct FSManager {
    footprint_path: Utf8PathBuf,
    pub footprint: Footprint,
}

impl FSManager {
    pub fn init(env: &Environment) -> FSManager {
        let footprint_path = env.footprint_path();
        let footprint = FSManager::read_and_parse_footprint(&footprint_path);

        FSManager {
            footprint_path,
            footprint,
        }
    }

    pub fn footprint_path(&self) -> &Utf8Path {
        &self.footprint_path
    }

    // ============================================================================
    // Filesystem Primitives (no footprint changes)
    // ============================================================================

    /// Creates a symlink on the filesystem
    pub fn create_symlink(&self, link: &Link) -> io::Result<()> {
        unix::fs::symlink(&link.src.path, &link.dest.path)?;
        Ok(())
    }

    /// Removes a symlink from the filesystem
    pub fn remove_symlink(&self, link: &Link) -> io::Result<()> {
        fs::remove_file(&link.dest.path)?;
        Ok(())
    }

    /// Creates a directory on the filesystem, returns true if it was created
    pub fn create_directory(&self, dir_path: &Utf8Path) -> io::Result<bool> {
        let existed = dir_path.exists();
        fs::create_dir_all(dir_path)?;
        Ok(!existed)
    }

    /// Removes a directory from the filesystem
    pub fn remove_directory(&self, dir_path: &Utf8Path) -> io::Result<()> {
        fs::remove_dir_all(dir_path)?;
        Ok(())
    }

    /// Removes a file from the filesystem
    pub fn remove_file(&self, file_path: &Utf8Path) -> io::Result<()> {
        fs::remove_file(file_path)?;
        Ok(())
    }

    /// Reads directory entries
    pub fn read_dir(&self, dir_path: &Utf8Path) -> io::Result<fs::ReadDir> {
        fs::read_dir(dir_path)
    }

    // ============================================================================
    // Footprint Primitives (no filesystem changes)
    // ============================================================================

    /// Adds a symlink to the footprint tracking
    pub fn track_symlink(&mut self, link: &Link) -> Result<()> {
        self.footprint.links.insert(link.clone());
        self.save_footprint()?;
        Ok(())
    }

    /// Adds a directory to the footprint tracking
    pub fn track_directory(&mut self, dir_path: &Utf8Path) -> Result<()> {
        self.footprint.dirs.insert(dir_path.to_path_buf());
        self.save_footprint()?;
        Ok(())
    }

    /// Edit the footprint with a closure, automatically saving changes when done
    pub fn edit_footprint<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Footprint),
    {
        f(&mut self.footprint);
        self.save_footprint()?;
        Ok(())
    }

    /// Write the current footprint to the toml file
    fn save_footprint(&self) -> Result<()> {
        let contents = toml::to_string(&self.footprint)?;
        fs::write(&self.footprint_path, contents)?;
        Ok(())
    }

    fn read_and_parse_footprint(footprint_path: &Utf8PathBuf) -> Footprint {
        let Ok(string) = fs::read_to_string(footprint_path) else {
            return Footprint::default();
        };
        return toml::from_str(string.as_ref()).unwrap_or_else(|err| {
            warn!("Error parsing {footprint_path}:\n{err}");
            Footprint::default()
        });
    }
}
