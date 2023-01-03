use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::{fs, io, os::unix};

use crate::{dots::Environment, footprint::Footprint, plan::links::Link};

pub struct FSManager {
    footprint_path: Utf8PathBuf,
    footprint: Footprint,
}

impl FSManager {
    pub fn init(env: &Environment) -> Result<FSManager> {
        let footprint_path = env.footprint_path();
        let footprint = FSManager::read_and_parse_footprint(&footprint_path)?;

        Ok(FSManager {
            footprint_path,
            footprint,
        })
    }

    /**
     * This method does three things:
     *
     * 1. removes any footprint links that DO NOT have corresponding symlinks on the fs
     * 2. removes any footprint links that DO have corresponding symlinks on the fs, but
     */
    pub fn clean(&mut self, valid_links: &Vec<Link>) -> Result<()> {
        let mut next_footprint = self.footprint.clone();
        for link in &self.footprint.links {
            let link_exists =
                link.dest.path.is_symlink() && (link.src.path.is_file() || link.src.path.is_dir());
            if !link_exists {
                next_footprint.links.remove(link);
            } else if !valid_links.contains(link) {
                self.remove_symlink(&link.src.path, &link.dest.path)?;
                next_footprint.links.remove(link);
            }
        }
        self.footprint = next_footprint;
        self.write_footprint()?;
        Ok(())
    }

    /**
     * Removes the given symlink from fs
     *
     * @todo should we take the src into account here and not remove the symlink if the source
     * doesn't match?
     */
    pub fn remove_symlink(&self, src: &Utf8Path, dest: &Utf8Path) -> io::Result<()> {
        fs::remove_file(dest)?;
        Ok(())
    }

    /** Creates the given symlink and tracks that link in the dot footprint */
    pub fn create_symlink(&mut self, src: &Utf8Path, dest: &Utf8Path) -> Result<()> {
        unix::fs::symlink(src, dest)?;
        self.footprint.links.insert(Link::new(src, dest));
        self.write_footprint()?;

        Ok(())
    }

    /** Write the given contents to the the dot footprint */
    fn write_footprint(&self) -> Result<()> {
        let contents = toml::to_string(&self.footprint)?;
        fs::write(&self.footprint_path, contents)?;
        Ok(())
    }

    fn read_and_parse_footprint(footprint_path: &Utf8PathBuf) -> Result<Footprint> {
        let Ok(string) = fs::read_to_string(footprint_path) else {
            return Ok(Footprint::default())
        };
        return toml::from_str(string.as_ref()).or_else(|err| {
            warn!("Error parsing {footprint_path}:\n{err}");
            Ok(Footprint::default())
        });
    }
}
