use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::{fs, io, os::unix};

use crate::{dots::Environment, footprint::Footprint, plan::links::Link};

pub struct FSManager {
    footprint_path: Utf8PathBuf,
    footprint: Footprint,
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

    /**
     * This method does three things:
     *
     * 1. removes any footprint links that DO NOT have corresponding symlinks on the fs
     * 2. removes any footprint links that DO have corresponding symlinks on the fs, but those symlinks
     *    do no point to the correct source.
     * 3. removes the symlink for footprint links that DO NOT have corresponding links in any Dot.toml
     */
    pub fn clean(&mut self, valid_links: &Vec<Link>, env: &Environment) -> Result<()> {
        let original_footprint = self.footprint.clone();
        debug!("VALID LINKS");
        for link in valid_links {
            debug!("  {link:?}")
        }
        debug!("FOOTPRINT LINKS");
        for link in &original_footprint.links {
            let symlink_exists = link.dest.path.is_symlink();
            debug!("  {link:?}");
            debug!("    link is on fs: {}", link.exists());
            debug!("    link is in dots: {}", valid_links.contains(link));
            if !symlink_exists {
                debug!("    no symlink detected, removing footprint link");
                self.remove_footprint_link(link)?;
            } else if !link.exists() {
                debug!("    symlink detected, but pointing to wrong dest, removing footprint link");
                self.remove_footprint_link(link)?;
            } else if !link.src.path.starts_with(env.root()) {
                debug!("    symlink exists, but source is outside of dots dir, removing footprint link");
                self.remove_footprint_link(link)?;
            } else if !valid_links.contains(link) {
                debug!("    link is on fs but is no longer present in dot files, removing symlink & footprint link");
                self.remove_symlink(&link.src.path, &link.dest.path)?;
                self.remove_footprint_link(link)?;
            } else {
                debug!("    leaving link alone")
            }
        }
        Ok(())
    }

    /** Removes a link from the footprint file */
    fn remove_footprint_link(&mut self, link: &Link) -> Result<()> {
        self.footprint.links.remove(link);
        self.save_footprint()?;
        Ok(())
    }

    /**
     * Removes the given symlink from fs
     */
    pub fn remove_symlink(&self, dest: &Utf8Path) -> io::Result<()> {
        fs::remove_file(dest)?;
        Ok(())
    }

    /** Creates the given symlink and tracks that link in the dot footprint */
    pub fn create_symlink(&mut self, src: &Utf8Path, dest: &Utf8Path) -> Result<()> {
        unix::fs::symlink(src, dest)?;
        self.footprint.links.insert(Link::new(src, dest));
        self.save_footprint()?;

        Ok(())
    }

    /** Write the given contents to the the dot footprint */
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
