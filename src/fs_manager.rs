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

    pub fn remove_symlink(&self, src: &Utf8Path, dest: &Utf8Path) -> io::Result<()> {
        fs::remove_file(dest)?;
        Ok(())
    }

    pub fn create_symlink(&mut self, src: &Utf8Path, dest: &Utf8Path) -> Result<()> {
        unix::fs::symlink(src, dest)?;
        self.footprint.links.push(footprint::FootprintLink {
            src: src.to_path_buf(),
            dest: dest.to_path_buf(),
        });

        let contents = toml::to_string(&self.footprint)?;
        fs::write(&self.footprint_path, contents)?;

        Ok(())
    }
}
