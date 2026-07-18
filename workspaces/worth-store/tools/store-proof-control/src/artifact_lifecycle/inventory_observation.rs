use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::evidence::{sha256_file, sha256_serialized};

use super::inventory::BuildArtifactRecord;
use super::{
    AdmittedArtifactRoot, BuildArtifactClass, BuildArtifactKind, DISPOSABLE_ARTIFACT_ROOT_MARKER,
};

#[derive(Serialize)]
struct FilesystemIdentityBasis<'a> {
    target_root: &'a str,
    rows: Vec<FilesystemIdentityRow<'a>>,
}

#[derive(Serialize)]
struct FilesystemIdentityRow<'a> {
    relative_path: &'a str,
    kind: BuildArtifactKind,
    logical_bytes: u64,
    modified_unix_nanos: u128,
}

pub(super) fn current_filesystem_identity(
    workspace_root: &Path,
    target_root: &Path,
) -> Result<String, String> {
    let admitted = AdmittedArtifactRoot::admit(workspace_root, target_root)?;
    let records = observe_records(&admitted, &BTreeSet::new(), false)?;
    filesystem_identity(target_root, &records)
}

pub(super) fn observe_records(
    admitted: &AdmittedArtifactRoot,
    current_paths: &BTreeSet<String>,
    has_reuse_basis: bool,
) -> Result<Vec<BuildArtifactRecord>, String> {
    observe_paths(admitted.target_root())?
        .into_iter()
        .map(|path| record(admitted, &path, current_paths, has_reuse_basis))
        .collect()
}

fn observe_paths(target_root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    let mut pending = vec![target_root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| error.to_string())?;
            let metadata = std::fs::symlink_metadata(entry.path()).map_err(|error| {
                format!("could not inspect {}: {error}", entry.path().display())
            })?;
            if metadata.file_type().is_symlink() {
                return Err(format!(
                    "artifact inventory denies symlink or junction {}",
                    entry.path().display()
                ));
            }
            if metadata.is_dir() {
                pending.push(entry.path());
            }
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths)
}

fn record(
    admitted: &AdmittedArtifactRoot,
    path: &Path,
    current_paths: &BTreeSet<String>,
    has_reuse_basis: bool,
) -> Result<BuildArtifactRecord, String> {
    let metadata = std::fs::symlink_metadata(path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    let relative = path.strip_prefix(admitted.target_root()).map_err(|_| {
        format!(
            "artifact escaped admitted root during inventory: {}",
            path.display()
        )
    })?;
    let absolute = normalized(path);
    let is_current = current_paths.contains(&absolute);
    let class = classify(relative, metadata.is_dir(), is_current, has_reuse_basis);
    let (protected, protection_reason) = protection(class, is_current, has_reuse_basis);
    let logical_bytes = metadata.is_file().then_some(metadata.len()).unwrap_or(0);
    let content_sha256 = if metadata.is_file() && removable_candidate(class, protected) {
        Some(sha256_file(path)?)
    } else {
        None
    };
    Ok(BuildArtifactRecord {
        relative_path: normalized(relative),
        absolute_path: absolute,
        class,
        kind: if metadata.is_dir() {
            BuildArtifactKind::Directory
        } else {
            BuildArtifactKind::File
        },
        logical_bytes,
        modified_unix_nanos: modified_unix_nanos(&metadata),
        content_sha256,
        protected,
        protection_reason,
    })
}

fn classify(
    relative: &Path,
    is_directory: bool,
    is_current: bool,
    has_reuse_basis: bool,
) -> BuildArtifactClass {
    let components: Vec<_> = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect();
    let file_name = relative
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let extension = relative
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if file_name == DISPOSABLE_ARTIFACT_ROOT_MARKER {
        return BuildArtifactClass::DisposableRootMarker;
    }
    if in_evidence_root(&components) {
        if components.iter().any(|component| component.contains("ui")) {
            return BuildArtifactClass::UiExpectation;
        }
        if components
            .iter()
            .any(|component| component.contains("process"))
        {
            return BuildArtifactClass::ProcessOutput;
        }
        if ["stdout", "stderr", "log"].contains(&extension.as_str()) {
            return BuildArtifactClass::DiagnosticCapture;
        }
        return BuildArtifactClass::EvidenceBundle;
    }
    if components
        .iter()
        .any(|component| component == "incremental")
    {
        return BuildArtifactClass::IncrementalState;
    }
    if extension == "pdb" {
        return BuildArtifactClass::Symbol;
    }
    if is_current {
        return BuildArtifactClass::CurrentReusableObject;
    }
    if !is_directory && has_reuse_basis && is_hashed_build_variant(&components, file_name) {
        return BuildArtifactClass::StaleHashedVariant;
    }
    BuildArtifactClass::UnattributedBuildArtifact
}

fn protection(
    class: BuildArtifactClass,
    is_current: bool,
    has_reuse_basis: bool,
) -> (bool, Option<String>) {
    if is_current {
        return (true, Some("protected by the bound proof run".to_owned()));
    }
    match class {
        BuildArtifactClass::EvidenceBundle
        | BuildArtifactClass::UiExpectation
        | BuildArtifactClass::ProcessOutput
        | BuildArtifactClass::DiagnosticCapture
        | BuildArtifactClass::DisposableRootMarker => {
            (true, Some("non-cache evidence lifecycle".to_owned()))
        }
        BuildArtifactClass::UnattributedBuildArtifact if !has_reuse_basis => (
            true,
            Some("no proof run establishes stale-vs-current status".to_owned()),
        ),
        _ => (false, None),
    }
}

fn removable_candidate(class: BuildArtifactClass, protected: bool) -> bool {
    !protected
        && matches!(
            class,
            BuildArtifactClass::StaleHashedVariant
                | BuildArtifactClass::IncrementalState
                | BuildArtifactClass::Symbol
        )
}

fn in_evidence_root(components: &[String]) -> bool {
    components
        .windows(2)
        .any(|pair| pair[0] == ".store-proof" && pair[1] == "evidence")
        || components
            .first()
            .is_some_and(|component| component == "evidence")
}

fn is_hashed_build_variant(components: &[String], file_name: &str) -> bool {
    let in_hashed_directory = components
        .iter()
        .any(|component| component == "deps" || component == "examples");
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default();
    let suffix = stem.rsplit_once('-').map(|(_, suffix)| suffix);
    in_hashed_directory
        && suffix.is_some_and(|suffix| {
            suffix.len() >= 8
                && suffix
                    .chars()
                    .all(|character| character.is_ascii_hexdigit())
        })
}

pub(super) fn filesystem_identity(
    target_root: &Path,
    artifacts: &[BuildArtifactRecord],
) -> Result<String, String> {
    let normalized_target = normalized(target_root);
    sha256_serialized(&FilesystemIdentityBasis {
        target_root: &normalized_target,
        rows: artifacts
            .iter()
            .map(|artifact| FilesystemIdentityRow {
                relative_path: &artifact.relative_path,
                kind: artifact.kind,
                logical_bytes: artifact.logical_bytes,
                modified_unix_nanos: artifact.modified_unix_nanos,
            })
            .collect(),
    })
}

fn modified_unix_nanos(metadata: &std::fs::Metadata) -> u128 {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
