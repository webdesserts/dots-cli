/**
 * # Plan Module
 *
 * The Plan orchestrates dotfiles operations through three phases:
 * clean (footprint maintenance) → validate (conflict detection) → execute (installation).
 *
 * Uses the Actions system for filesystem operations to enable dry-run support,
 * error collection, and operation inspection.
 */
use crate::dots::{Dot, Environment};
use crate::fs_manager::FSManager;
use crate::plan::resolve::{ResolveIssueKind, ResolvedLink};
use anyhow::Result;
use camino::Utf8Path;
use std::{
    collections::HashSet,
    fmt::{self, Display},
    io,
};

use super::action::Action;
use super::links::Link;
use super::resolve::{ResolveIssue, ResolveIssueLevel};

mod styles {
    use utils::stylize::Style;

    pub const TITLE: Style = Style::new().bold();
}

#[derive(Debug)]
pub struct PlanError {
    msg: String,
}

impl PlanError {
    fn new(msg: &str) -> PlanError {
        PlanError {
            msg: msg.to_string(),
        }
    }
}

impl Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plan Error: {}", self.msg)
    }
}

impl std::error::Error for PlanError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

/*======*\
*  Plan  *
\*======*/

pub struct Plan {
    force: bool,
    links: Vec<ResolvedLink>,
}

impl Plan {
    pub fn new(force: bool) -> Plan {
        Plan {
            force,
            links: vec![],
        }
    }

    /// Cleans up stale symlinks and empty directories, then reconciles the footprint.
    ///
    /// Generates and executes cleanup actions for:
    /// - Symlinks in footprint that no longer exist or point to wrong targets
    /// - Symlinks in footprint that aren't in current dot.toml files
    /// - Empty tracked directories that aren't needed by current links
    ///
    /// After cleanup, reconciles the footprint by removing stale entries and warning
    /// about directories that can't be removed due to user files.
    pub fn clean(
        &self,
        env: &Environment,
        fs_manager: &mut FSManager,
        dots: &[Dot],
        dry_run: bool,
    ) -> Result<()> {
        let current_links: Vec<Link> = dots
            .iter()
            .flat_map(|dot| &dot.links)
            .filter_map(|resolved_link| resolved_link.as_link())
            .collect();

        // Generate cleanup actions
        let cleanup_actions = self.generate_cleanup_actions(env, fs_manager, &current_links);

        if dry_run {
            // Just log what would be cleaned up
            debug!("DRY RUN - Cleanup actions that would be executed:");
            for action in &cleanup_actions {
                debug!("  {:?}", action);
            }
        } else {
            // Execute cleanup actions
            debug!("CLEANUP ACTIONS");
            for action in cleanup_actions {
                let should_skip = action.should_skip();
                debug!("skip: {}, action: {:?}", should_skip, action);
                if !should_skip {
                    match action.execute(fs_manager) {
                        Ok(_) => {}
                        Err(err) => {
                            // Log error but continue with other cleanup actions
                            warn!("Cleanup action failed: {}", err);
                            warn!("You may need to clean up manually");
                        }
                    }
                }
            }
        }

        // Reconcile footprint after cleanup
        // Check for directories with user files before reconciliation
        let dirs_with_user_files: Vec<_> = fs_manager
            .footprint
            .dirs
            .iter()
            .filter(|dir_path| {
                let is_needed = current_links
                    .iter()
                    .any(|link| link.dest.path.starts_with(dir_path));

                if !is_needed && dir_path.is_dir() {
                    // Check if has user files
                    fs_manager
                        .read_dir(dir_path)
                        .ok()
                        .map(|entries| {
                            entries.filter_map(|e| e.ok()).any(|entry| {
                                let path = entry.path();
                                path.is_file() && !path.is_symlink()
                            })
                        })
                        .unwrap_or(false)
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        // Warn about directories we can't remove
        for dir_path in &dirs_with_user_files {
            warn!(
                "Cannot remove directory {} - it contains files that were not created by dots",
                utils::fs::pretty_path(dir_path)
            );
        }

        fs_manager.edit_footprint(|footprint| {
            // Remove links that don't exist or point outside dots directory
            footprint.links.retain(|link| {
                link.dest.path.is_symlink() && link.src.path.starts_with(env.root())
            });

            // Remove directories that don't exist or aren't needed by current links
            footprint.dirs.retain(|dir_path| {
                let dir_exists = dir_path.is_dir();
                let is_needed = current_links
                    .iter()
                    .any(|link| link.dest.path.starts_with(dir_path));

                dir_exists && is_needed
            });
        })?;

        Ok(())
    }

    /// Validates the dotfiles installation plan and detects conflicts.
    ///
    /// Uses the ResolvedLink system to check for missing files, conflicting symlinks,
    /// permission issues, and duplicate destinations. Shows warnings and errors
    /// to the user before any filesystem changes are made.
    ///
    /// Returns Ok if the plan can proceed, or Err if there are unresolved issues.
    pub fn validate(&mut self, dots: &[Dot]) -> Result<(), PlanError> {
        let mut suggest_force = false;
        let mut fixed_issues: Vec<&ResolveIssue> = vec![];
        for (index, dot) in dots.iter().enumerate() {
            let title = format!("[{name}]", name = &dot.package.name);
            if index > 0 {
                eprintln!();
            }
            eprintln!("{title}", title = styles::TITLE.apply(title));
            let links = &dot.links;

            for link in links {
                let mut link = link.clone();
                if let Some(resolved_dest) = &link.dest.path {
                    let duplicates = self.duplicates(resolved_dest);
                    if !duplicates.is_empty() {
                        link.dest.mark_as_duplicate();
                    }
                }

                eprintln!("{link}");
                self.links.push(link);
            }
        }

        let issues = self.issues();

        if !issues.is_empty() {
            let existing_file_issues: Vec<&ResolveIssue> = self
                .issues()
                .into_iter()
                .filter(|&issue| matches!(issue.kind, ResolveIssueKind::AlreadyExists(_)))
                .collect();

            let has_existing_files = !existing_file_issues.is_empty();

            if self.force {
                for issue in existing_file_issues {
                    fixed_issues.push(issue);
                }
            }

            if !self.force && has_existing_files {
                suggest_force = true;
            }

            if issues.len() > fixed_issues.len() {
                eprintln!();
            }

            for issue in issues {
                use crate::plan::resolve::ResolveIssueLevel::*;
                match issue.level() {
                    Error => error!("{issue}"),
                    Warning => {
                        if !fixed_issues.contains(&issue) {
                            warn!("{issue}")
                        }
                    }
                }
            }
        }

        eprintln!();

        if suggest_force {
            info!("use --force to overwrite existing directories");
            eprintln!();
        }

        if self.has_errors() {
            Err(PlanError::new("Planning failed."))
        } else if self.has_warnings()
            && self
                .warnings()
                .into_iter()
                .any(|warning| !fixed_issues.contains(&warning))
        {
            Err(PlanError::new("Plan has unresolved warnings."))
        } else {
            Ok(())
        }
    }

    /// Executes the dotfiles installation using the Actions system.
    ///
    /// Trusts that validation has passed and performs the actual filesystem operations
    /// to install dotfiles. Uses Actions to create directories, symlinks, and remove
    /// conflicting files (when force=true).
    ///
    /// Collects multiple errors instead of failing fast to provide comprehensive
    /// feedback about what went wrong during installation.
    pub fn execute(
        &self,
        _env: &Environment,
        fs_manager: &mut FSManager,
        force: bool,
    ) -> Result<()> {
        let links: Vec<Link> = self
            .links
            .iter()
            .filter_map(|resolved_link| resolved_link.as_link())
            .collect();

        let mut actions: Vec<Action> = vec![];

        for link in &links {
            if link.dest.path.is_symlink() {
                actions.push(Action::RemoveLink(link.clone()));
            } else if link.dest.path.is_file() {
                if !force {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Destination already Exists!",
                    )));
                }

                actions.push(Action::RemoveFile(link.dest.path.clone()));
            } else if link.dest.path.is_dir() {
                if !force {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Destination already Exists!",
                    )));
                }

                actions.push(Action::RemoveDir(link.dest.path.clone()));
            }

            if let Some(parent) = link.dest.path.parent() {
                actions.push(Action::CreateDir(parent.to_owned()));
            }

            actions.push(Action::CreateLink(link.clone()));
        }

        debug!("ACTIONS");

        /*
         * TODO:
         * I want a list of errors produced from different actions
         * I want a list of actions produced from different links
         * can we have 1 action connected to multiple links?
         * if so that means we can have 1 error connected to multiple links as well
         */

        let mut errors: Vec<anyhow::Error> = vec![];

        for action in actions {
            let should_skip = action.should_skip();
            debug!("skip: {}, action: {:?}", should_skip, action);
            if should_skip {
                continue;
            }
            if let Some(error) = action.execute(fs_manager).err() {
                errors.push(error)
            }
        }

        if let Some(error) = errors.pop() {
            return Err(error);
        }

        Ok(())
    }

    fn duplicates(&self, path: &Utf8Path) -> Vec<&ResolvedLink> {
        self.links
            .iter()
            .filter(|&link| link.dest.path == Some(path.to_path_buf()))
            .collect()
    }

    fn issues(&self) -> Vec<&ResolveIssue> {
        self.links.iter().flat_map(|link| link.issues()).collect()
    }

    fn has_errors(&self) -> bool {
        self.links.iter().any(|link| link.has_errors())
    }

    fn has_warnings(&self) -> bool {
        self.links.iter().any(|link| link.has_warnings())
    }

    fn warnings(&self) -> Vec<&ResolveIssue> {
        self.issues()
            .into_iter()
            .filter(|&issue| matches!(issue.level(), ResolveIssueLevel::Warning))
            .collect()
    }

    #[allow(unused)]
    fn errors(&self) -> Vec<&ResolveIssue> {
        self.issues()
            .into_iter()
            .filter(|&issue| matches!(issue.level(), ResolveIssueLevel::Error))
            .collect()
    }

    /// Generates cleanup actions for stale symlinks and empty directories.
    ///
    /// Returns actions that need to be executed to clean up the filesystem based on:
    /// - Symlinks in footprint that don't exist anymore or point to wrong targets
    /// - Symlinks in footprint that aren't in current dot.toml files
    /// - Empty tracked directories that aren't needed by current links
    fn generate_cleanup_actions(
        &self,
        env: &Environment,
        fs_manager: &FSManager,
        current_links: &[Link],
    ) -> Vec<Action> {
        let mut actions = vec![];

        // Collect all destination paths from current links for quick lookup
        let current_dest_paths: HashSet<_> = current_links
            .iter()
            .map(|link| &link.dest.path)
            .collect();

        // Clean up stale symlinks
        for footprint_link in &fs_manager.footprint.links {
            // Skip links that point outside dots directory - they're not ours to manage
            if !footprint_link.src.path.starts_with(env.root()) {
                continue;
            }

            if footprint_link.dest.path.is_symlink() {
                // Check if this destination path is covered by a current link.
                // If so, we should NOT remove it - the current link will recreate/update it.
                // This handles the case where a link's source path changed in Dot.toml.
                if current_dest_paths.contains(&footprint_link.dest.path) {
                    continue;
                }

                if !footprint_link.exists() {
                    // Stale symlink pointing to wrong target
                    actions.push(Action::RemoveLink(footprint_link.clone()));
                } else if !current_links.contains(footprint_link) {
                    // Symlink not in current dot.toml
                    actions.push(Action::RemoveLink(footprint_link.clone()));
                }
            }
        }

        // Clean up empty directories (sort by depth, deepest first)
        let mut tracked_dirs: Vec<_> = fs_manager.footprint.dirs.iter().collect();
        tracked_dirs.sort_by_key(|dir| std::cmp::Reverse(dir.components().count()));

        for dir_path in tracked_dirs {
            if dir_path.is_dir() && !Self::directory_contains_files(dir_path, fs_manager) {
                // Directory is empty and exists
                // Check if any current links still need this directory
                let still_needed = current_links
                    .iter()
                    .any(|link| link.dest.path.starts_with(dir_path));

                if !still_needed {
                    actions.push(Action::RemoveDir(dir_path.clone()));
                }
            }
        }

        actions
    }

    /// Checks if a directory contains any files (not empty or only has subdirs)
    fn directory_contains_files(dir_path: &Utf8Path, fs_manager: &FSManager) -> bool {
        if let Ok(entries) = fs_manager.read_dir(dir_path) {
            entries.count() > 0
        } else {
            false
        }
    }
}
