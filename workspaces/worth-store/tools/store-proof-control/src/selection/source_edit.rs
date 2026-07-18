use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_file;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestedSourceEdit {
    pub source_path: String,
    pub original_sha256: String,
    pub purpose: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservedSourceEditIdentity {
    pub source_path: String,
    pub original_sha256: String,
    pub edited_sha256: String,
    pub purpose: String,
    pub description: String,
}

impl RequestedSourceEdit {
    pub fn new(
        source_path: String,
        original_sha256: String,
        purpose: String,
        description: String,
    ) -> Self {
        Self {
            source_path,
            original_sha256,
            purpose,
            description,
        }
    }
}

pub(super) fn observe(
    workspace_root: &Path,
    requested: Option<&RequestedSourceEdit>,
) -> Result<Option<ObservedSourceEditIdentity>, String> {
    requested
        .map(|requested| observe_one(workspace_root, requested))
        .transpose()
}

fn observe_one(
    workspace_root: &Path,
    requested: &RequestedSourceEdit,
) -> Result<ObservedSourceEditIdentity, String> {
    let relative = Path::new(&requested.source_path);
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
        || requested.source_path.trim().is_empty()
        || requested.purpose.trim().is_empty()
        || requested.description.trim().is_empty()
        || !is_sha256(&requested.original_sha256)
    {
        return Err("source edit declaration is incomplete or escapes the workspace".to_owned());
    }
    let workspace = workspace_root.canonicalize().map_err(|error| {
        format!(
            "could not resolve workspace root {}: {error}",
            workspace_root.display()
        )
    })?;
    let path = workspace_root.join(relative);
    let metadata = std::fs::symlink_metadata(&path).map_err(|error| {
        format!(
            "could not inspect edited source {}: {error}",
            path.display()
        )
    })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "edited source is not a regular file: {}",
            path.display()
        ));
    }
    let canonical = path.canonicalize().map_err(|error| {
        format!(
            "could not resolve edited source {}: {error}",
            path.display()
        )
    })?;
    if !canonical.starts_with(&workspace) {
        return Err(format!(
            "edited source escaped the workspace: {}",
            path.display()
        ));
    }
    let edited_sha256 = sha256_file(&canonical)?;
    if edited_sha256 == requested.original_sha256 {
        return Err("source edit declaration points at unchanged content".to_owned());
    }
    Ok(ObservedSourceEditIdentity {
        source_path: requested.source_path.replace('\\', "/"),
        original_sha256: requested.original_sha256.clone(),
        edited_sha256,
        purpose: requested.purpose.clone(),
        description: requested.description.clone(),
    })
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
