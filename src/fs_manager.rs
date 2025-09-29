use anyhow::Result;
use camino::Utf8PathBuf;
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


    /** Removes a link from the footprint file */
    pub fn remove_footprint_link(&mut self, link: &Link) -> Result<()> {
        self.footprint.links.remove(link);
        self.save_footprint()?;
        Ok(())
    }

    /**
     * Removes the given symlink from fs
     */
    pub fn remove_symlink(&self, link: &Link) -> io::Result<()> {
        fs::remove_file(&link.dest.path)?;
        Ok(())
    }

    /** Creates the given symlink and tracks that link in the dot footprint */
    pub fn create_symlink(&mut self, link: &Link) -> Result<()> {
        unix::fs::symlink(&link.src.path, &link.dest.path)?;
        self.footprint.links.insert(link.clone());
        self.save_footprint()?;

        Ok(())
    }

    /** Removes a directory from the footprint tracking */
    pub fn remove_footprint_dir(&mut self, dir_path: &Utf8PathBuf) -> Result<()> {
        self.footprint.dirs.remove(dir_path);
        self.save_footprint()?;
        Ok(())
    }

    /** Creates a directory and tracks it if it didn't exist before */
    pub fn create_directory(&mut self, dir_path: &Utf8PathBuf) -> Result<()> {
        let existed = dir_path.exists();
        fs::create_dir_all(dir_path)?;
        // Only track directories that we actually created
        if !existed {
            self.footprint.dirs.insert(dir_path.clone());
            self.save_footprint()?;
        }
        Ok(())
    }

    /** Removes a file from the filesystem */
    pub fn remove_file(&self, file_path: &Utf8PathBuf) -> io::Result<()> {
        fs::remove_file(file_path)?;
        Ok(())
    }

    /** Removes a directory and all its contents from the filesystem */
    pub fn remove_directory(&self, dir_path: &Utf8PathBuf) -> io::Result<()> {
        fs::remove_dir_all(dir_path)?;
        Ok(())
    }

    /** Write the current footprint to the toml file */
    fn save_footprint(&self) -> Result<()> {
        let contents = toml::to_string(&self.footprint)?;
        fs::write(&self.footprint_path, contents)?;
        Ok(())
    }

    fn read_and_parse_footprint(footprint_path: &Utf8PathBuf) -> Footprint {
        let Ok(string) = fs::read_to_string(footprint_path) else {
            return Footprint::default()
        };
        return toml::from_str(string.as_ref()).unwrap_or_else(|err| {
            warn!("Error parsing {footprint_path}:\n{err}");
            Footprint::default()
        });
    }
}
