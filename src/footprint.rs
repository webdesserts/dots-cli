use camino::Utf8PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Footprint {
    /// @todo Consider changing this to a Set
    pub links: Vec<FootprintLink>,
}

#[derive(Serialize, Deserialize)]
pub struct FootprintLink {
    /// An absolute path to the dotfile
    pub src: Utf8PathBuf,
    /// An absolute path to the symlink
    pub dest: Utf8PathBuf,
}
