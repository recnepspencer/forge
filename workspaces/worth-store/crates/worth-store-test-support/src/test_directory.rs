use std::path::Path;

/// A filesystem root whose lifetime is exactly the lifetime of its test fixture.
#[derive(Clone, Debug)]
pub struct TemporaryDirectory {
    directory: std::sync::Arc<tempfile::TempDir>,
}

impl TemporaryDirectory {
    pub fn create(label: &str) -> std::io::Result<Self> {
        tempfile::Builder::new()
            .prefix(&format!("worth-store-{label}-"))
            .tempdir()
            .map(|directory| Self {
                directory: std::sync::Arc::new(directory),
            })
    }

    pub fn path(&self) -> &Path {
        self.directory.path()
    }
}

#[cfg(test)]
mod tests {
    use super::TemporaryDirectory;

    #[test]
    fn dropping_fixture_removes_its_filesystem_root() {
        let path = {
            let directory = TemporaryDirectory::create("lifetime-test").unwrap();
            let path = directory.path().to_owned();
            std::fs::write(path.join("owned"), b"data").unwrap();
            path
        };

        assert!(!path.exists());
    }
}
