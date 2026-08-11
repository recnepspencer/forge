use super::super::evidence::S0StableDigest;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum S0ScanScopeRejection {
    EmptyRequiredField,
    AbsolutePath,
    ParentTraversal,
    WorkspaceGlobalScope,
    ForbiddenGeneratedScope,
    NoDeclaredScanRoots,
    DuplicateDeclaredRoot,
    DuplicateMatchedFile,
    MatchedFileOutsideDeclaredRoots,
    MissingDigest,
    DigestConstructionFailed,
    StaleSchemaVersion,
    StaleSourceRevision,
    StaleManifestDigest,
}

pub(super) fn require_non_empty(value: impl Into<String>) -> Result<String, S0ScanScopeRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0ScanScopeRejection::EmptyRequiredField);
    }
    Ok(value.trim().to_string())
}

pub(super) fn normalize_path(value: impl Into<String>) -> Result<String, S0ScanScopeRejection> {
    let value = require_non_empty(value)?;
    let normalized = value.replace('\\', "/");
    if normalized.starts_with('/') || normalized.contains(":/") {
        return Err(S0ScanScopeRejection::AbsolutePath);
    }
    if normalized.split('/').any(|part| part == "..") {
        return Err(S0ScanScopeRejection::ParentTraversal);
    }
    Ok(normalized.trim_matches('/').to_string())
}

pub(super) fn is_forbidden_scope(path: &str) -> bool {
    path.split('/').any(|part| {
        matches!(
            part,
            "target" | ".git" | "node_modules" | "vendor" | "generated" | "dist"
        )
    })
}

pub(super) fn reject_duplicate_roots<T: AsRef<str>>(
    roots: impl IntoIterator<Item = T>,
) -> Result<(), S0ScanScopeRejection> {
    let mut seen = BTreeSet::new();
    if roots
        .into_iter()
        .any(|root| !seen.insert(root.as_ref().to_string()))
    {
        return Err(S0ScanScopeRejection::DuplicateDeclaredRoot);
    }
    Ok(())
}

pub(super) fn reject_duplicate_files<T: AsRef<str>>(
    files: impl IntoIterator<Item = T>,
) -> Result<(), S0ScanScopeRejection> {
    let mut seen = BTreeSet::new();
    if files
        .into_iter()
        .any(|file| !seen.insert(file.as_ref().to_string()))
    {
        return Err(S0ScanScopeRejection::DuplicateMatchedFile);
    }
    Ok(())
}

pub(super) fn file_is_under_root(file_path: &str, root_path: &str) -> bool {
    file_path == root_path
        || file_path
            .strip_prefix(root_path)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, S0ScanScopeRejection> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| S0ScanScopeRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0ScanScopeRejection::DigestConstructionFailed)
}
