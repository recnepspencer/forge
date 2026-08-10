//! Temporary governed repository writer for authority sealing proofs.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

mod assembly;
mod configuration;
mod entry_crate;
mod stub_dependencies;

/// Isolated temporary repository exercising the production boundary-check binary.
pub struct AuthoritySealingTestRepository {
    pub(super) root: PathBuf,
}

impl AuthoritySealingTestRepository {
    pub fn create(label: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "boundary-check-authority-sealing-{label}-{}-{nanos}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        Self { root }
    }

    pub fn write_file(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent directories");
        }
        fs::write(path, contents).expect("write file");
    }
}
