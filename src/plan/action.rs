use anyhow::Context;
use camino::Utf8PathBuf;

use crate::fs_manager::FSManager;

use super::links;

/**
 * # Actions System
 *
 * Actions represent high-level filesystem operations with built-in intelligence.
 * They serve as an abstraction layer over raw filesystem calls, providing:
 *
 * - **Dry-run support**: Generate and inspect actions without executing them
 * - **Error collection**: Collect multiple errors instead of failing fast
 * - **Operation inspection**: Debug what filesystem changes will be made
 * - **Error avoidance**: Skip operations that would produce expected errors
 *
 * ## Design Philosophy:
 * - Actions are **high-level filesystem operations**, not low-level system calls
 * - Each action can validate itself and decide if it should be skipped
 * - Actions are generic and can be used by any component needing filesystem operations
 * - Actions enable better user experience through inspection and error collection
 */
#[derive(Debug)]
pub enum Action {
    CreateDir(Utf8PathBuf),
    CreateLink(links::Link),
    RemoveFile(Utf8PathBuf),
    RemoveDir(Utf8PathBuf),
    RemoveLink(links::Link),
}

impl Action {
    /// Executes the filesystem operation represented by this action.
    ///
    /// Orchestrates FSManager primitives to perform filesystem changes and footprint tracking:
    /// - Creations (CreateDir, CreateLink) are always tracked in the footprint
    /// - Removals (RemoveLink, RemoveDir, RemoveFile) only touch filesystem, footprint is reconciled later
    pub fn execute(&self, fs: &mut FSManager) -> anyhow::Result<()> {
        use Action::*;

        match self {
            CreateDir(path) => {
                let was_created = fs.create_directory(path)
                    .with_context(|| format!("Failed to create directory {}", path))?;
                if was_created {
                    fs.track_directory(path)
                        .with_context(|| format!("Failed to update {}", utils::fs::pretty_path(fs.footprint_path())))?;
                }
            }
            CreateLink(link) => {
                fs.create_symlink(link)
                    .with_context(|| format!("Failed to create symlink {}", link))?;
                fs.track_symlink(link)
                    .with_context(|| format!("Failed to update {}", utils::fs::pretty_path(fs.footprint_path())))?;
            }
            RemoveFile(path) => {
                fs.remove_file(path)
                    .with_context(|| format!("Failed to remove file {}", path))?;
            }
            RemoveDir(path) => {
                fs.remove_directory(path)
                    .with_context(|| format!("Failed to remove directory {}", path))?;
            }
            RemoveLink(link) => {
                fs.remove_symlink(link)
                    .with_context(|| format!("Failed to remove symlink at {}", utils::fs::pretty_path(&link.dest.path)))?;
            }
        };
        Ok(())
    }

    /// Determines if this action should be skipped to avoid expected filesystem errors.
    ///
    /// This method checks the current filesystem state to avoid operations that would
    /// produce "expected" errors that we'd have to filter out later (like NotFound
    /// when removing non-existent files, or AlreadyExists when creating existing dirs).
    ///
    /// This is primarily about error avoidance, not performance - we want to collect
    /// only "real" errors, not expected errors from no-op operations.
    pub fn should_skip(&self) -> bool {
        use Action::*;
        match self {
            CreateDir(path) => path.is_dir(),
            CreateLink(link) => link.exists(),
            RemoveFile(path) => !path.is_file(),
            RemoveDir(path) => !path.is_dir(),
            RemoveLink(link) => !link.dest.exists(),
        }
    }
}
