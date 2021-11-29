use crate::dots::Dot;
use crate::plan::links::Link;
use crate::plan::resolve::{resolve, ResolvedLink};
use camino::Utf8Path;
use colored::*;
use std::error::Error;
use std::{
    fmt::{self, Display},
    io,
    rc::Rc,
};

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
    pub fn new(dots: Vec<Dot>, _force: bool) -> Result<Plan, PlanError> {
        use colored::*;

        let suggest_force = false;
        let mut plan = Plan { requests: vec![] };
        let mut has_errors = false;

        for dot in dots {
            let title = format!("[{}]", &dot.package.package.name);
            println!("\n{}", title.bold());
            let links = dot.package.link.clone();
            let dot = Rc::new(dot);

            for (src, dest) in links {
                let request = LinkRequest::new(Rc::clone(&dot), src, dest);
                if request.has_errors() {
                    has_errors = true
                }
                println!("{}", request);
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

    pub fn execute(&self, _force: bool) -> io::Result<()> {
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
    dot: Rc<Dot>,
    link: ResolvedLink,
}

impl Display for LinkRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::plan::resolve::ResolveIssueLevel::*;
        let checkmark = "✔".green();
        let cross = "✖".red();
        let src = &self.link.src;
        let dest = &self.link.dest;

        let statusmark = if src.has_errors() | dest.has_errors() {
            cross
        } else {
            checkmark
        };

        let src_path = format!("{}", src.original.path);
        let src_msg = match src.max_issue_level() {
            Some(Error) => src_path.red().italic().to_string(),
            Some(Warning) => src_path.yellow().underline().to_string(),
            None => src_path,
        };

        let dest_path = format!("{}", dest.original.path);
        let dest_msg = match dest.max_issue_level() {
            Some(Error) => format!("{}", dest_path.red().italic()),
            Some(Warning) => format!("{}", dest_path.yellow().underline()),
            None => dest_path,
        };

        write!(f, "{} {} => {}", statusmark, src_msg, dest_msg)
    }
}

impl LinkRequest {
    fn new<P>(dot: Rc<Dot>, src: P, dest: P) -> Self
    where
        P: AsRef<Utf8Path>,
    {
        let link = resolve(&dot, Link::new(src, dest));
        LinkRequest { dot, link }
    }

    fn has_errors(&self) -> bool {
        self.link.src.has_errors() | self.link.dest.has_errors()
    }
}

#[cfg(test)]
mod tests {
    mod link_request {
        use crate::dots::Dot;

        #[test]
        fn it_should_display_correctly() -> Result<(), failure::Error> {
            let dot = Dot::new("./fixtures/example_dot/")?;
            assert_eq!(dot.path, "./fixtures/example_dot/");
            assert_eq!(dot.package.package.name, "example_package");
            Ok(())
        }
    }
}
