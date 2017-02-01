#[macro_use]
extern crate clap;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = clap_app!(dots =>
        (version: VERSION)
        (author: "Michael Mullins <michael@webdesserts.com>")
        (about: "Just connecting the dot(file)s")
        (@subcommand install =>
            (about: "Installs and links all files and necessary binaries")
        )
        (@subcommand doctor =>
            (about: "Checks to make sure all files and symlinks are correctly applied")
        )
        (@subcommand update =>
            (about: "Updates all dotfiles")
        )
    );

    let matches = app.get_matches();

    match matches.subcommand_name() {
      Some("install") => { println!("install has not yet been implemented!") },
      Some("doctor") => { println!("doctor has not yet been implemented!") },
      Some("update") => { println!("update has not yet been implemented!") },
      _              => {},
    }
}
