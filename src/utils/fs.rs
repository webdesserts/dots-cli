use std::fs;
use std::path::Path;

pub fn clean(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}
