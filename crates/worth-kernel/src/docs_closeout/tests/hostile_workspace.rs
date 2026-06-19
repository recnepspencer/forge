use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const TOUCHED_CRATES: [&str; 4] = ["worth-kernel", "worth-spatial", "worth-topo", "worth-geom"];

pub struct WorthDocsTestWorkspace {
    root: PathBuf,
}

impl WorthDocsTestWorkspace {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-docs-closeout-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos()
        ));
        fs::create_dir_all(&root).expect("temp workspace root should build");
        let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("worth-kernel should live under crates/")
            .to_path_buf();
        for crate_name in TOUCHED_CRATES {
            copy_dir_all(
                &source_root.join("crates").join(crate_name).join("docs"),
                &root.join("crates").join(crate_name).join("docs"),
            );
        }
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn read_file(&self, relative_path: &str) -> String {
        fs::read_to_string(self.root.join(relative_path)).expect("file should exist")
    }

    pub fn write_file(&self, relative_path: &str, contents: impl AsRef<str>) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directory should build");
        }
        fs::write(path, contents.as_ref()).expect("file should write");
    }

    pub fn replace_once(&self, relative_path: &str, from: &str, to: &str) {
        let contents = self.read_file(relative_path);
        let updated = contents.replacen(from, to, 1);
        assert_ne!(updated, contents, "replacement target should exist");
        self.write_file(relative_path, updated);
    }

    pub fn copy_file(&self, from_relative_path: &str, to_relative_path: &str) {
        let source = self.root.join(from_relative_path);
        let destination = self.root.join(to_relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).expect("destination parent should build");
        }
        fs::copy(source, destination).expect("file should copy");
    }

    pub fn rename_path(&self, from_relative_path: &str, to_relative_path: &str) {
        let source = self.root.join(from_relative_path);
        let destination = self.root.join(to_relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).expect("destination parent should build");
        }
        fs::rename(source, destination).expect("path should rename");
    }

    pub fn remove_line(&self, relative_path: &str, needle: &str) {
        let updated = self
            .read_file(relative_path)
            .lines()
            .filter(|line| !line.contains(needle))
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
            .join("\n");
        self.write_file(relative_path, format!("{updated}\n"));
    }

    pub fn remove_directory(&self, relative_path: &str) {
        fs::remove_dir_all(self.root.join(relative_path)).expect("directory should remove");
    }
}

impl Drop for WorthDocsTestWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn copy_dir_all(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination should build");
    for entry in fs::read_dir(source).expect("source directory should exist") {
        let entry = entry.expect("directory entry should read");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry
            .file_type()
            .expect("directory entry type should read")
            .is_dir()
        {
            copy_dir_all(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("file should copy");
        }
    }
}
