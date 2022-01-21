use dots::Dot;
use plan::actions::{Action, ResolveErrorKind};
use plan::links::{Anchor, AnchorKind};
use std::error::Error;
use std::fmt::Display;
use std::{self, fmt, fs, io};

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

impl Error for PlanError {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}

pub struct Plan {
    pub actions: Vec<Action>,
}

impl Plan {
    pub fn new(dots: Vec<Dot>, force: bool) -> Result<Plan, PlanError> {
        use colored::*;

        let mut suggest_force = false;
        let mut plan = Plan { actions: vec![] };
        let mut errors = vec![];

        for dot in dots {
            let title = format!("[{}]", &dot.package.package.name);
            println!("\n{}", title.bold());

            for (src, dest) in dot.package.link {
                let requested_action = Action::Link {
                    src: Anchor::new(src, AnchorKind::Source),
                    dest: Anchor::new(dest, AnchorKind::Destination),
                };

                match requested_action.resolve(&dot.path, &force) {
                    Ok(action) => {
                        println!("  {}", requested_action);
                        plan.actions.push(action)
                    }
                    Err(err) => {
                        match err.kind {
                            ResolveErrorKind::AlreadyExists => suggest_force = true,
                            _ => {}
                        }

                        println!("  {}", requested_action.display_err(&err));
                        errors.push(err)
                    }
                }
            }
        }

        let has_errors = !errors.is_empty();

        if has_errors {
            println!()
        }
        for err in errors {
            error!("{}", err)
        }

        println!();
        if suggest_force {
            info!("{}", "use --force to overwrite existing directories")
        }
        if has_errors {
            Err(PlanError::new("Planning failed."))
        } else {
            Ok(plan)
        }
    }

    pub fn execute(&self, force: bool) -> io::Result<()> {
        for action in &self.actions {
            match *action {
                Action::Link { ref src, ref dest } => {
                    if dest.path.exists() {
                        let file_type = dest.path.metadata()?.file_type();
                        if file_type.is_symlink() || file_type.is_file() {
                            fs::remove_file(&dest.path)?;
                        } else if file_type.is_dir() {
                            if force {
                                fs::remove_dir_all(&dest.path)?;
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::AlreadyExists,
                                    "Destination already Exists!",
                                ));
                            }
                        };
                    };

                    if let Some(parent) = dest.path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    std::os::unix::fs::symlink(&src.path, &dest.path)?;
                }
            }
        }
        Ok(())
    }
}
