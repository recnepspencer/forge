use super::manifest_validation::{
    is_forbidden_scope, normalize_path, require_non_empty, S0ScanScopeRejection,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct S0DeclaredScanRoot {
    root: String,
    purpose: String,
}

impl S0DeclaredScanRoot {
    pub fn new(
        root: impl Into<String>,
        purpose: impl Into<String>,
    ) -> Result<Self, S0ScanScopeRejection> {
        let root = normalize_path(root)?;
        if root == "." || root == "*" || root.is_empty() {
            return Err(S0ScanScopeRejection::WorkspaceGlobalScope);
        }
        if is_forbidden_scope(&root) {
            return Err(S0ScanScopeRejection::ForbiddenGeneratedScope);
        }
        let purpose = require_non_empty(purpose)?;
        Ok(Self { root, purpose })
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn purpose(&self) -> &str {
        &self.purpose
    }
}
