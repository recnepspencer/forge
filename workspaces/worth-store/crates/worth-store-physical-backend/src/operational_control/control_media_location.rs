use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlMediaLocation(PathBuf);

impl ControlMediaLocation {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn recovery_object_root(&self) -> PathBuf {
        self.sibling_with_suffix(".objects")
    }

    pub fn identity_path(&self) -> PathBuf {
        self.sibling_with_suffix(".identity")
    }

    fn sibling_with_suffix(&self, suffix: &str) -> PathBuf {
        let mut name = self
            .0
            .file_name()
            .map_or_else(|| OsString::from("control"), OsString::from);
        name.push(suffix);
        self.0.with_file_name(name)
    }
}
