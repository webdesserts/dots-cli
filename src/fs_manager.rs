use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::{fs, io, os::unix};

use crate::{
    dots::Environment,
    footprint::{self, Footprint},
};

pub struct FSManager {
    footprint_path: Utf8PathBuf,
    footprint: Footprint,
}

impl FSManager {
    pub fn init(env: &Environment) -> FSManager {
        let footprint = Footprint { links: vec![] };
        let footprint_path = env.footprint_path();
        FSManager {
            footprint_path,
            footprint,
        }
    }

    /**
     * Removes the given symlink from fs
     *
     * @todo should we take the src into account here and not remove the symlink if the source
     * doesn't match?
     *
     */
    pub fn remove_symlink(&self, src: &Utf8Path, dest: &Utf8Path) -> io::Result<()> {
        fs::remove_file(dest)?;
        Ok(())
    }

    /** Creates the given symlink and tracks that link in the dot footprint */
    pub fn create_symlink(&mut self, src: &Utf8Path, dest: &Utf8Path) -> Result<()> {
        unix::fs::symlink(src, dest)?;
        self.footprint.links.push(footprint::FootprintLink {
            src: src.to_path_buf(),
            dest: dest.to_path_buf(),
        });

        self.write_footprint()?;

        Ok(())
    }

    /** Write the given contents to the the dot footprint */
    fn write_footprint(&self) -> Result<()> {
        let contents = toml::to_string(&self.footprint)?;
        fs::write(&self.footprint_path, contents)?;
        Ok(())
    }
}
