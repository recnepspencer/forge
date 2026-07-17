use std::path::{Component, Path};

use super::RecoverySourceLeaseDenial;

pub(super) fn record_name(path: &Path) -> Result<String, RecoverySourceLeaseDenial> {
    if !is_safe_relative_path(path) {
        return Err(RecoverySourceLeaseDenial::InvalidArtifactName);
    }
    path.to_str()
        .map(str::to_owned)
        .ok_or(RecoverySourceLeaseDenial::InvalidArtifactName)
}

pub(super) fn validate_source_artifact(
    source_root: &Path,
    record_name: &str,
) -> Result<(), RecoverySourceLeaseDenial> {
    let relative = Path::new(record_name);
    if !is_safe_relative_path(relative) {
        return Err(RecoverySourceLeaseDenial::InvalidArtifactName);
    }
    let declared = source_root.join(relative);
    let metadata = std::fs::symlink_metadata(&declared).map_err(|_| {
        RecoverySourceLeaseDenial::MissingSourceArtifact {
            output_name: record_name.to_owned(),
        }
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(RecoverySourceLeaseDenial::InvalidArtifactName);
    }
    let canonical = std::fs::canonicalize(&declared)?;
    if !canonical.starts_with(source_root) {
        return Err(RecoverySourceLeaseDenial::InvalidArtifactName);
    }
    Ok(())
}

fn is_safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
