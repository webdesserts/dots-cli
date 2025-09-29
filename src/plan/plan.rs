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

    /**
     * Cleans up stale footprint entries and broken symlinks.
     *
     * This method performs footprint maintenance by:
     * 1. Removing footprint entries for symlinks that were manually deleted
     * 2. Removing stale symlinks that point to incorrect targets and their footprint entries
     * 3. Removing footprint entries for symlinks pointing outside the dots directory
     * 4. Removing symlinks that are no longer present in any dot.toml and their footprint entries
     */
    pub fn clean(&self, env: &Environment, fs_manager: &mut FSManager, dots: &[Dot]) -> Result<()> {
        let links: Vec<Link> = dots
            .iter()
            .flat_map(|dot| &dot.links)
            .filter_map(|resolved_link| resolved_link.as_link())
            .collect();

        let mut fs_actions: Vec<Action> = vec![];
        let mut footprint_removals: Vec<Link> = vec![];

        // Generate cleanup actions based on footprint vs reality
        for footprint_link in fs_manager.footprint.links.iter().cloned() {
            if !footprint_link.dest.path.is_symlink() {
                debug!("no symlink detected, removing footprint link");
                // Symlink was deleted manually, just clean footprint
                footprint_removals.push(footprint_link);
            } else if !footprint_link.exists() {
                debug!("symlink detected, but pointing to wrong dest, removing symlink and footprint link");
                // Stale symlink pointing to wrong target, remove it and clean footprint
                fs_actions.push(Action::RemoveLink(footprint_link.clone()));
                footprint_removals.push(footprint_link);
            } else if !footprint_link.src.path.starts_with(env.root()) {
                debug!("symlink exists, but source is outside of dots dir, removing footprint link");
                // Symlink points outside dots directory, just clean footprint
                footprint_removals.push(footprint_link);
            } else if !links.contains(&footprint_link) {
                debug!("symlink exists, but not in any dot toml, removing symlink and footprint link");
                // Symlink exists but not in any current dot.toml, remove it and clean footprint
                fs_actions.push(Action::RemoveLink(footprint_link.clone()));
                footprint_removals.push(footprint_link);
            }
        }

        debug!("CLEANUP ACTIONS");

        // Execute filesystem actions with skip logic
        for action in fs_actions {
            let should_skip = action.should_skip();
            debug!("skip: {}, action: {:?}", should_skip, action);
            if !should_skip {
                action.execute(fs_manager)?;
            }
        }

        // Clean up tracked directories that are now empty
        let mut tracked_dirs: Vec<_> = fs_manager.footprint.dirs.iter().cloned().collect();
        // Sort by depth (deepest first) to handle nested directories correctly
        tracked_dirs.sort_by_key(|b| std::cmp::Reverse(b.components().count()));

        let mut dir_removals: Vec<camino::Utf8PathBuf> = vec![];
        for dir_path in tracked_dirs {
            if dir_path.is_dir() {
                match std::fs::read_dir(&dir_path) {
                    Ok(mut entries) => {
                        if entries.next().is_none() {
                            // Directory is empty - try to remove
                            let remove_action = Action::RemoveDir(dir_path.clone());
                            if !remove_action.should_skip() {
                                match remove_action.execute(fs_manager) {
                                    Ok(_) => {
                                        // Successfully removed
                                        dir_removals.push(dir_path);
                                    }
                                    Err(err) => {
                                        // Failed to remove empty directory (permissions?)
                                        warn!("Unable to remove empty directory {}: {}", dir_path, err);
                                        warn!("You may need to remove it manually with appropriate permissions");
                                        // Keep tracking - don't add to dir_removals
                                    }
                                }
                            }
                        } else {
                            // Directory contains files - stop tracking it
                            // The user has claimed this directory for their own use
                            dir_removals.push(dir_path);
                        }
                    }
                    Err(err) => {
                        // Can't read directory (permissions?)
                        warn!("Unable to access directory {} for cleanup: {}", dir_path, err);
                        warn!("You may need to check permissions or remove it manually");
                        // Keep tracking - don't add to dir_removals
                    }
                }
            } else {
                // Directory doesn't exist - stop tracking
                dir_removals.push(dir_path);
            }
        }

        // Update footprint entries directly
        for link in footprint_removals {
            fs_manager.remove_footprint_link(&link)?;
        }

        // Remove cleaned directories from footprint tracking
        for dir_path in dir_removals {
            fs_manager.remove_footprint_dir(&dir_path)?;
        }
        Ok(())
    }

    /**
     * Validates the dotfiles installation plan and detects conflicts.
     *
     * Uses the ResolvedLink system to check for missing files, conflicting symlinks,
     * permission issues, and duplicate destinations. Shows warnings and errors
     * to the user before any filesystem changes are made.
     *
     * Returns Ok if the plan can proceed, or Err if there are unresolved issues.
     */
    pub fn validate(&mut self, dots: Vec<Dot>) -> Result<(), PlanError> {
        let mut suggest_force = false;
        let mut fixed_issues: Vec<&ResolveIssue> = vec![];
        for dot in dots {
            let title = format!("[{name}]", name = &dot.package.name);
            eprintln!("\n{title}", title = styles::TITLE.apply(title));
            let links = dot.links;

            for mut link in links {
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

    /**
     * Executes the dotfiles installation using the Actions system.
     *
     * Trusts that validation has passed and performs the actual filesystem operations
     * to install dotfiles. Uses Actions to create directories, symlinks, and remove
     * conflicting files (when force=true).
     *
     * Collects multiple errors instead of failing fast to provide comprehensive
     * feedback about what went wrong during installation.
     */
    pub fn execute(&self, fs_manager: &mut FSManager, force: bool) -> Result<()> {
        let links: Vec<Link> = self
            .links
            .iter()
            .filter_map(|resolved_link| resolved_link.as_link())
            .collect();

        let mut actions: Vec<Action> = vec![];

        for link in links {
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
}
