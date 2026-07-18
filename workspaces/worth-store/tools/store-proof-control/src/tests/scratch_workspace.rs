use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) struct ScratchCargoWorkspace {
    root: PathBuf,
}

impl ScratchCargoWorkspace {
    pub(super) fn new(responsibility: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("scratch workspace clock follows Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "worth-store-proof-control-{responsibility}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("scratch workspace root is creatable");
        let status = Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(&root)
            .status()
            .expect("git is available to discovery fixtures");
        assert!(status.success(), "scratch workspace git init failed");
        let workspace = Self { root };
        workspace.write(
            "test-control/feature-semantic-authority.json",
            "{\"schema_version\":1,\"declarations\":[]}\n",
        );
        workspace
    }

    pub(super) fn root(&self) -> &Path {
        &self.root
    }

    pub(super) fn write(&self, relative: &str, source: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("scratch source parent is creatable");
        }
        fs::write(path, source).expect("scratch source is writable");
    }
}

impl Drop for ScratchCargoWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
