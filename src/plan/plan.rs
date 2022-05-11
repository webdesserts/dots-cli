use crate::dots::{Dot, Environment};
use crate::fs_manager::FSManager;
use crate::plan::links::Link;
use crate::plan::resolve::{resolve, ResolveIssueKind, ResolvedLink};
use anyhow::Result;
use camino::Utf8Path;
use std::fs;
use std::{
    fmt::{self, Display},
    io,
};
use utils::stylize::Stylable;

use super::resolve::{ResolveIssue, ResolveIssueLevel};

mod styles {
    use utils::{style, stylize::Style};

    pub const TITLE: Style = style! { Bold };
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
    pub links: Vec<ResolvedLink>,
}

impl Plan {
    pub fn new(dots: Vec<Dot>, force: &bool) -> Result<Plan, PlanError> {
        let mut suggest_force = false;
        let mut plan = Plan { links: vec![] };
        let mut fixed_issues: Vec<&ResolveIssue> = vec![];

        for dot in dots {
            let title = format!("[{name}]", name = &dot.package.package.name);
            eprintln!("\n{title}", title = title.apply_style(styles::TITLE));
            let links = dot.package.link.clone();

            for (src, dest) in links {
                let link = Link::new(src, dest);
                let mut resolved_link = resolve(&dot, link);
                if let Some(resolved_dest) = &resolved_link.dest.path {
                    let duplicates = plan.duplicates(resolved_dest);
                    if !duplicates.is_empty() {
                        resolved_link.dest.mark_as_duplicate();
                    }
                }

                eprintln!("{resolved_link}");
                plan.links.push(resolved_link);
            }
        }

        let issues = plan.issues();

        if !issues.is_empty() {
            let existing_file_issues: Vec<&ResolveIssue> = plan
                .issues()
                .into_iter()
                .filter(|&issue| matches!(issue.kind, ResolveIssueKind::AlreadyExists(_)))
                .collect();

            let has_existing_files = !existing_file_issues.is_empty();

            if *force {
                for issue in existing_file_issues {
                    fixed_issues.push(issue);
                }
            }

            if !*force && has_existing_files {
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

        if plan.has_errors() {
            Err(PlanError::new("Planning failed."))
        } else if plan.has_warnings()
            && plan
                .warnings()
                .into_iter()
                .any(|warning| !fixed_issues.contains(&warning))
        {
            Err(PlanError::new("Plan has unresolved warnings."))
        } else {
            Ok(plan)
        }
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

    fn errors(&self) -> Vec<&ResolveIssue> {
        self.issues()
            .into_iter()
            .filter(|&issue| matches!(issue.level(), ResolveIssueLevel::Error))
            .collect()
    }

    pub fn execute(&self, env: &Environment, force: &bool) -> Result<()> {
        let mut fs_manager = FSManager::init(env);
        for link in &self.links {
            let src = match &link.src.path {
                Some(path) => path,
                None => continue,
            };
            let dest = match &link.dest.path {
                Some(path) => path,
                None => continue,
            };

            if dest.is_symlink() {
                fs_manager.remove_symlink(src, dest)?;
            } else if dest.is_file() {
                if !force {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Destination already Exists!",
                    )));
                }

                fs::remove_file(&dest)?;
            } else if dest.is_dir() {
                if !force {
                    return Err(anyhow::Error::new(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Destination already Exists!",
                    )));
                }

                fs::remove_dir_all(&dest)?;
            }

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }

            fs_manager.create_symlink(src, dest)?;
        }
        Ok(())
    }
}
