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
    /**
     * Executes the filesystem operation represented by this action.
     *
     * Performs the actual filesystem changes. Operations that affect symlinks
     * (CreateLink/RemoveLink) automatically update the footprint tracking.
     */
    pub fn execute(&self, fs: &mut FSManager) -> anyhow::Result<()> {
        use Action::*;

        match self {
            CreateDir(path) => std::fs::create_dir_all(path)?,
            CreateLink(link) => fs.create_symlink(link)?,
            RemoveFile(path) => std::fs::remove_file(path)?,
            RemoveDir(path) => std::fs::remove_dir_all(path)?,
            RemoveLink(link) => fs.remove_symlink(link)?,
        };
        Ok(())
    }

    /**
     * Determines if this action should be skipped to avoid expected filesystem errors.
     *
     * This method checks the current filesystem state to avoid operations that would
     * produce "expected" errors that we'd have to filter out later (like NotFound
     * when removing non-existent files, or AlreadyExists when creating existing dirs).
     *
     * This is primarily about error avoidance, not performance - we want to collect
     * only "real" errors, not expected errors from no-op operations.
     */
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
