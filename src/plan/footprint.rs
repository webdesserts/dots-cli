use std::path::PathBuf;

/*============*\
*  DotHistory  *
\*============*/

pub struct DotFootprint {
    links: Vec<LinkRecord>,
    dirs: Vec<PathBuf>
}

struct LinkRecord {
    src: PathBuf,
    dest: PathBuf,
}

pub struct InstallInfo {
    footprint: Map<String, DotFootprint>
}
