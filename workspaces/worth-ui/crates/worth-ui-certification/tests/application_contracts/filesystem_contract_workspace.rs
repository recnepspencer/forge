use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use atomicwrites::replace_atomic;

static NEXT_WORKSPACE: AtomicU64 = AtomicU64::new(1);

pub(super) struct FilesystemContractWorkspace {
    root: Option<PathBuf>,
}

impl FilesystemContractWorkspace {
    pub(super) fn new(responsibility: &str) -> Self {
        let sequence = NEXT_WORKSPACE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "worth-ui-{responsibility}-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("contract workspace root should be unique and creatable");
        Self { root: Some(root) }
    }

    pub(super) fn root(&self) -> &Path {
        self.root.as_deref().expect("open contract workspace")
    }

    pub(super) fn path(&self, relative_path: &str) -> PathBuf {
        self.root().join(relative_path)
    }

    pub(super) fn write(&self, relative_path: &str, source: &str) {
        let path = self.path(relative_path);
        let parent = path
            .parent()
            .expect("contract source path should retain a parent");
        fs::create_dir_all(parent).expect("contract source parent should be creatable");
        fs::write(path, source).expect("external editor simulation should write source bytes");
    }

    pub(super) fn write_atomic(&self, relative_path: &str, source: &str) {
        let path = self.path(relative_path);
        let parent = path
            .parent()
            .expect("contract source path should retain a parent");
        fs::create_dir_all(parent).expect("contract source parent should be creatable");
        let pending = path.with_extension("wui.pending");
        fs::write(&pending, source).expect("editor should write pending replacement bytes");
        replace_atomic(&pending, &path)
            .expect("editor should atomically replace the published source on one filesystem");
    }

    pub(super) fn remove(&self, relative_path: &str) {
        fs::remove_file(self.path(relative_path))
            .expect("external editor simulation should remove source bytes");
    }

    pub(super) fn close(mut self) {
        let root = self.root.take().expect("open contract workspace");
        fs::remove_dir_all(&root).unwrap_or_else(|error| {
            panic!(
                "filesystem contract workspace cleanup failed for {}: {error}",
                root.display()
            )
        });
    }
}

impl Drop for FilesystemContractWorkspace {
    fn drop(&mut self) {
        if let Some(root) = self.root.take() {
            let _ = fs::remove_dir_all(root);
        }
    }
}
