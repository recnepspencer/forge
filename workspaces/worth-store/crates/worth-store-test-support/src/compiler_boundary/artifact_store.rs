use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{UiFixtureRunEvidence, UiProofRunEvidence, UiProofRunFailure, UI_EVIDENCE_ROOT_ENV};

pub(super) fn admitted_evidence_root(workspace_root: &Path) -> Result<PathBuf, UiProofRunFailure> {
    let default = workspace_root.join(".store-proof/evidence/ui");
    let Some(declared) = std::env::var_os(UI_EVIDENCE_ROOT_ENV).map(PathBuf::from) else {
        return Ok(default);
    };
    let declared = if declared.is_absolute() {
        declared
    } else {
        workspace_root.join(declared)
    };
    let admitted = workspace_root.join(".store-proof/evidence");
    if !declared.starts_with(&admitted) {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "{UI_EVIDENCE_ROOT_ENV} must remain under {}",
            admitted.display()
        )));
    }
    Ok(declared)
}

pub(super) fn write_immutable_file(path: &Path, bytes: &[u8]) -> Result<bool, UiProofRunFailure> {
    let parent = path.parent().ok_or_else(|| {
        UiProofRunFailure::EnvironmentObservation(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    publish_immutable(path, bytes).map_err(UiProofRunFailure::EnvironmentObservation)
}

pub(super) fn persist_fixture_evidence(
    path: &Path,
    evidence: &UiFixtureRunEvidence,
) -> Result<(), UiProofRunFailure> {
    persist_serialized(path, evidence, "checked diagnostic")
}

pub(super) fn persist_suite_evidence(
    path: &Path,
    evidence: &UiProofRunEvidence,
) -> Result<(), UiProofRunFailure> {
    persist_serialized(path, evidence, "UI run evidence")
}

fn persist_serialized(
    path: &Path,
    evidence: &impl serde::Serialize,
    evidence_kind: &str,
) -> Result<(), UiProofRunFailure> {
    let parent = path.parent().ok_or_else(|| {
        UiProofRunFailure::EvidenceWrite(format!("{} has no parent", path.display()))
    })?;
    fs::create_dir_all(parent)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    let mut encoded = serde_json::to_vec_pretty(evidence)
        .map_err(|error| UiProofRunFailure::EvidenceWrite(error.to_string()))?;
    encoded.push(b'\n');
    match publish_immutable(path, &encoded) {
        Ok(_) => Ok(()),
        Err(error) => Err(UiProofRunFailure::EvidenceWrite(format!(
            "{evidence_kind}: {error}"
        ))),
    }
}

fn publish_immutable(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent", path.display()))?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{} has no UTF-8 file name", path.display()))?;
    let nonce = PUBLISH_NONCE.fetch_add(1, Ordering::Relaxed);
    let staging = parent.join(format!(
        ".{file_name}.publish-{}-{nonce}",
        std::process::id()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)
        .map_err(|error| format!("create {}: {error}", staging.display()))?;
    let write = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = write {
        let _ = fs::remove_file(&staging);
        return Err(format!("write {}: {error}", staging.display()));
    }
    let linked = fs::hard_link(&staging, path);
    let _ = fs::remove_file(&staging);
    match linked {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(path)
                .map_err(|error| format!("inspect {}: {error}", path.display()))?;
            if !metadata.is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "immutable UI artifact is not a regular file: {}",
                    path.display()
                ));
            }
            let existing =
                fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
            if existing == bytes {
                Ok(false)
            } else {
                Err(format!(
                    "immutable UI artifact identity collision at {}",
                    path.display()
                ))
            }
        }
        Err(error) => Err(format!("publish {}: {error}", path.display())),
    }
}

static PUBLISH_NONCE: AtomicU64 = AtomicU64::new(0);
