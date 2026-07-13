use std::fs;
use std::path::Path;

pub fn recreate(root: &Path) {
    let _ = fs::remove_dir_all(root);
}

pub fn write(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("parent")).expect("create fixture directory");
    fs::write(path, contents).expect("write fixture file");
}
