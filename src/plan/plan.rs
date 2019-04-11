use colored::*;
use dots::Dot;
use plan::links::{Anchor, AnchorKind, Link};
use plan::resolve::{resolve, ResolvedLink};
use std::error::Error;
use std::fmt::Display;
use std::path::Path;
use std::{self, fmt, fs, io};

/*
## TODOs

- rework plan to use new resolve structure
- Figure out what to do with the Footprint
- Figure out if a "root" directory should be a part of Anchor::Source
- Figure out a way to make all of this testable, this is getting out of hand
    - Make .dots path configurable (probably moved to tmp somewhere)
    - Make "home directory" configurable (so we aren't still linking relative to it)

--- vvv Original TODOs (almost complete?) vvv ---

- Figure out how to structure, store, and handle Request Errors & Warnings
- Figure out how to convert LinkRequests into a flat array of actions
- Figure out how to continue gathering errors when something errors out.
- Figure out how the footprint should be stored and plan a way get all the way
  from a LinkRequest to a Footprint

## What's in a link?
A link needs to be used to make the actual symlink
A link needs to be printed out
A link needs to print out differently based on errors & warnings
A link needs to be stored in a dotfootprint

## Errors & Warnings
There are two types of errors:
Errors resolving the source of a link
Errors resolving the destination of a link

Warnings – issues that we can resolve, but should require user confirmation.
The install should still be resumable once a warning is confirmed.
(e.g Are you sure you want to overwrite this directory?)

Errors – issues that we can't control or issues that need to be resolved by
the user. Errors should stop the install in its tracks.

We always want to return an Array of Errors & Warnings even if it's empty
*/

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

/*======*\
*  Plan  *
\*======*/

pub struct Plan {
    pub requests: Vec<LinkRequest>,
}

impl Plan {
    pub fn new(dots: Vec<Dot>, force: bool) -> Result<Plan, PlanError> {
        use colored::*;

        let mut suggest_force = false;
        let mut plan = Plan { requests: vec![] };
        let mut has_errors = false;

        for dot in dots {
            let title = format!("[{}]", &dot.package.package.name);
            println!("\n{}", title.bold());

            for (src, dest) in dot.package.link {
                let request = LinkRequest::new(dot, src, dest);
                if request.has_errors() {
                    has_errors = true
                }
                println!("\n{}", request);
                plan.requests.push(request);
            }
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
        // for action in &self.actions {
        //     match *action {
        //         ResolvedLink { ref src, ref dest } => {
        //             if dest.path.exists() {
        //                 let file_type = dest.path.metadata()?.file_type();
        //                 if file_type.is_symlink() || file_type.is_file() {
        //                     fs::remove_file(&dest.path)?;
        //                 } else if file_type.is_dir() {
        //                     if force {
        //                         fs::remove_dir_all(&dest.path)?;
        //                     } else {
        //                         return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Destination already Exists!"));
        //                     }
        //                 };
        //             };

        //             if let Some(parent) = dest.path.parent() {
        //                 fs::create_dir_all(parent)?;
        //             }
        //             std::os::unix::fs::symlink(&src.path, &dest.path)?;
        //         }
        //     }
        // }
        Ok(())
    }
}

/*===============*\
*  Link Requests  *
\*===============*/

pub struct LinkRequest {
    dot: Dot,
    link: ResolvedLink,
}

impl Display for LinkRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use plan::resolve::ResolveIssueLevel::*;
        let checkmark = "✔".green();
        let cross = "✖".red();
        let src = self.link.src;
        let dest = self.link.dest;
        let src_is_ok = src.has_errors();
        let dest_is_ok = dest.has_errors();

        let statusmark = if src.has_errors() | dest.has_errors() {
            cross
        } else {
            checkmark
        };

        let src_path = format!("{}", src.original.path.display());
        let src_msg = match src.max_issue_level() {
            Some(Error) => src_path.red().italic().to_string(),
            Some(Warning) => src_path.yellow().underline().to_string(),
            None => src_path,
        };

        let dest_path = format!("{}", dest.original.path.display());
        let dest_msg = match dest.max_issue_level() {
            Some(Error) => format!("{}", dest_path.red().italic()),
            Some(Warning) => format!("{}", dest_path.yellow().underline()),
            None => dest_path,
        };

        write!(f, "{} {} => {}", statusmark, src_msg, dest_msg)
    }
}

impl LinkRequest {
    fn new<P: AsRef<Path>>(dot: Dot, src: P, dest: P) -> Self {
        let link = Link::new(src, dest);
        LinkRequest {
            dot,
            link: resolve(dot, link),
        }
    }

    fn has_errors(&self) -> bool {
        self.link.src.has_errors() | self.link.dest.has_errors()
    }
}
