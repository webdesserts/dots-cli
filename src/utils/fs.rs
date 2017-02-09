use std::path::Path;
use std::fs;

pub fn clean(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).ok();
    }
}
