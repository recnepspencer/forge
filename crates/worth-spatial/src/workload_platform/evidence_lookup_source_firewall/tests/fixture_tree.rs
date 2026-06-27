use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct SourceFirewallFixtureTree {
    root: PathBuf,
}

impl SourceFirewallFixtureTree {
    pub(crate) fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "worth-spatial-evidence-lookup-source-firewall-{unique}"
        ));
        fs::create_dir_all(&root).expect("fixture root directory");
        Self { root }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn write_file(&self, relative_path: &str, source: &str) {
        let file_path = self.root.join(relative_path);
        let parent = file_path.parent().expect("fixture file parent");
        fs::create_dir_all(parent).expect("fixture parent directory");
        fs::write(file_path, source).expect("fixture file write");
    }
}

impl Drop for SourceFirewallFixtureTree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
