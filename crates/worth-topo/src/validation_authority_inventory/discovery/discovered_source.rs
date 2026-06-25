use std::path::{Path, PathBuf};

use super::scan_pattern::WorthValidationAuthorityScanPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityDiscoveredSource {
    path: PathBuf,
    pattern: WorthValidationAuthorityScanPattern,
}

impl WorthValidationAuthorityDiscoveredSource {
    pub(super) fn from_parts(path: PathBuf, pattern: WorthValidationAuthorityScanPattern) -> Self {
        Self { path, pattern }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn pattern(&self) -> WorthValidationAuthorityScanPattern {
        self.pattern
    }

    pub fn normalized_path(&self) -> String {
        self.path.to_string_lossy().replace('\\', "/")
    }
}
