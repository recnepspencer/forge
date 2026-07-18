use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloseoutArtifactReference {
    pub repository_relative_path: String,
    pub sha256: String,
}

impl CloseoutArtifactReference {
    pub(super) fn validate(&self) -> Result<(), String> {
        let path = Path::new(&self.repository_relative_path);
        if path.is_absolute()
            || self.repository_relative_path.trim().is_empty()
            || path
                .components()
                .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
            || self.sha256.len() != 64
            || !self.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(format!(
                "closeout artifact reference is invalid: {}",
                self.repository_relative_path
            ));
        }
        Ok(())
    }
}
