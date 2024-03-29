use crate::dots::{Dot, Environment};
use crate::fs_manager::FSManager;
use crate::plan::resolve::{ResolveIssueKind, ResolvedLink};
use anyhow::Result;
use camino::Utf8Path;
use std::fs;
use std::{
    fmt::{self, Display},
    io,
};

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

    pub fn clean(&self, env: &Environment, fs_manager: &mut FSManager, dots: &[Dot]) -> Result<()> {
        let links: Vec<Link> = dots
            .iter()
            .flat_map(|dot| &dot.links)
            .filter_map(|resolved_link| resolved_link.as_link())
            .collect();
        fs_manager.clean(&links, env)?;
        Ok(())
    }

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

    pub fn execute(&self, fs_manager: &mut FSManager, force: bool) -> Result<()> {
        let links: Vec<Link> = self
            .links
            .iter()
            .filter_map(|resolved_link| resolved_link.as_link())
            .collect();

        for link in links {
            if link.dest.path.is_symlink() {
                fs_manager.remove_symlink(&link)?;
            } else if link.dest.path.is_file() {
                if !force {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Destination already Exists!",
                    )));
                }

                fs::remove_file(&link.dest.path)?;
            } else if link.dest.path.is_dir() {
                if !force {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Destination already Exists!",
                    )));
                }

                fs::remove_dir_all(&link.dest.path)?;
            }

            if let Some(parent) = link.dest.path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs_manager.create_symlink(&link)?;
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
