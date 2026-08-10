use super::validation::{normalize_relative_path, TerminologyCleanupRejection};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TerminologyScanScope {
    path: String,
}

impl TerminologyScanScope {
    pub fn new(path: impl Into<String>) -> Result<Self, TerminologyCleanupRejection> {
        let path = normalize_relative_path(path)?;
        if path == "." || path == "./" || path.contains("..") {
            return Err(TerminologyCleanupRejection::RejectedWorkspaceGlobalScope);
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TerminologyScanPlan {
    scopes: Vec<TerminologyScanScope>,
}

impl TerminologyScanPlan {
    pub fn new(scopes: Vec<TerminologyScanScope>) -> Result<Self, TerminologyCleanupRejection> {
        if scopes.is_empty() {
            return Err(TerminologyCleanupRejection::MissingScanScope);
        }
        let mut seen = BTreeSet::new();
        if scopes.iter().any(|scope| !seen.insert(scope.path())) {
            return Err(TerminologyCleanupRejection::DuplicateScanScope);
        }
        Ok(Self { scopes })
    }

    pub fn scopes(&self) -> &[TerminologyScanScope] {
        &self.scopes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TerminologyScanInputFile {
    path: String,
    contents: String,
}

impl TerminologyScanInputFile {
    pub fn new(
        path: impl Into<String>,
        contents: impl Into<String>,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let path = normalize_relative_path(path)?;
        Ok(Self {
            path,
            contents: contents.into(),
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}
