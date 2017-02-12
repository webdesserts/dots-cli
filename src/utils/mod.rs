pub use self::git::{clone};
pub use self::fs::{clean};
pub use self::links::{Anchor, AnchorKind};

pub mod git;
pub mod fs;
pub mod links;
