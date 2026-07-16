use std::path::Path;

use worth_store_physical_format::BackupBundleManifest;

use super::BackupVerificationDefect;
use crate::{OfflineFileTruthEvidence, OfflineWalkedFile, StructurallyWalkedMedia};

pub(super) fn maximum_truth_evidence_owned_allocation_bytes(
    root: &Path,
    manifest: &BackupBundleManifest,
) -> Option<u64> {
    let entry_count = manifest.artifacts().len().checked_add(1)?;
    let rows = allocation_bytes::<OfflineFileTruthEvidence>(entry_count)?;
    let manifest_path = joined_path_payload_bytes(root, "backup.manifest")?;
    manifest
        .artifacts()
        .iter()
        .try_fold(rows.checked_add(manifest_path)?, |total, row| {
            total.checked_add(joined_path_payload_bytes(root, row.output_name())?)
        })
}

pub(super) fn maximum_structural_comparison_owned_allocation_bytes(
    manifest: &BackupBundleManifest,
    walked: &StructurallyWalkedMedia,
    defect_capacity: usize,
) -> Option<u64> {
    let defect_rows = allocation_bytes::<BackupVerificationDefect>(defect_capacity)?;
    let possible_name_payloads = manifest.artifacts().iter().try_fold(0_u64, |total, row| {
        total.checked_add(
            u64::try_from(row.output_name().len())
                .ok()?
                .checked_mul(5)?,
        )
    })?;
    let possible_path_payloads = walked.files().iter().try_fold(0_u64, |total, file| {
        total.checked_add(path_payload_bytes(file.path())?)
    })?;
    let possible_defects = defect_rows
        .checked_add(possible_name_payloads)?
        .checked_add(possible_path_payloads)?;
    let validation_workspace = allocation_bytes::<(u64, u64)>(manifest.artifacts().len())?;
    let comparison_indexes = allocation_bytes::<(&str, &OfflineWalkedFile)>(walked.files().len())?
        .checked_add(allocation_bytes::<&str>(manifest.artifacts().len())?)?;
    possible_defects.checked_add(validation_workspace.max(comparison_indexes))
}

pub(super) fn defect_owned_allocation_bytes(
    defects: &Vec<BackupVerificationDefect>,
) -> Option<u64> {
    defects.iter().try_fold(
        allocation_bytes::<BackupVerificationDefect>(defects.capacity())?,
        |total, defect| total.checked_add(defect_payload_bytes(defect)?),
    )
}

pub(super) fn structural_working_set_bytes(
    defects: &Vec<BackupVerificationDefect>,
    walked_name_index_capacity: usize,
    expected_name_index_capacity: usize,
) -> Option<u64> {
    let defect_rows = allocation_bytes::<BackupVerificationDefect>(defects.capacity())?;
    let walked_index_rows =
        allocation_bytes::<(&str, &OfflineWalkedFile)>(walked_name_index_capacity)?;
    let expected_index_rows = allocation_bytes::<&str>(expected_name_index_capacity)?;
    let row_storage = defect_rows
        .checked_add(walked_index_rows)?
        .checked_add(expected_index_rows)?;
    defects.iter().try_fold(row_storage, |total, defect| {
        total.checked_add(defect_payload_bytes(defect)?)
    })
}

pub(super) fn allocation_bytes<T>(capacity: usize) -> Option<u64> {
    u64::try_from(capacity)
        .ok()?
        .checked_mul(std::mem::size_of::<T>() as u64)
}

pub(super) fn defect_payload_bytes(defect: &BackupVerificationDefect) -> Option<u64> {
    use BackupVerificationDefect::*;
    match defect {
        MissingComponent { output_name }
        | ComponentLengthMismatch { output_name, .. }
        | ComponentDigestMismatch { output_name }
        | CoverageFamilyMismatch { output_name }
        | OwnerSemanticMismatch { output_name, .. } => u64::try_from(output_name.capacity()).ok(),
        ExtraComponent { path } => path_payload_bytes(path),
        VerificationCounterOverflow
        | PublishedManifestChanged
        | MissingArtifactFamily(_)
        | RootGenerationMismatch
        | CheckpointGenerationMismatch
        | RootCoverageMismatch
        | CheckpointCoverageMismatch
        | WalCoverageGapOrOverlap => Some(0),
    }
}

#[cfg(windows)]
fn path_payload_bytes(path: &Path) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    u64::try_from(path.as_os_str().encode_wide().count())
        .ok()?
        .checked_mul(2)
}

#[cfg(windows)]
fn joined_path_payload_bytes(root: &Path, output_name: &str) -> Option<u64> {
    use std::os::windows::ffi::OsStrExt;
    let root_units = u64::try_from(root.as_os_str().encode_wide().count()).ok()?;
    let name_units = u64::try_from(output_name.encode_utf16().count()).ok()?;
    root_units
        .checked_add(1)?
        .checked_add(name_units)?
        .checked_mul(2)
}

#[cfg(unix)]
fn joined_path_payload_bytes(root: &Path, output_name: &str) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    u64::try_from(root.as_os_str().as_bytes().len())
        .ok()?
        .checked_add(1)?
        .checked_add(u64::try_from(output_name.len()).ok()?)
}

#[cfg(not(any(windows, unix)))]
fn joined_path_payload_bytes(root: &Path, output_name: &str) -> Option<u64> {
    u64::try_from(root.to_string_lossy().len())
        .ok()?
        .checked_add(1)?
        .checked_add(u64::try_from(output_name.len()).ok()?)
}

#[cfg(unix)]
fn path_payload_bytes(path: &Path) -> Option<u64> {
    use std::os::unix::ffi::OsStrExt;
    u64::try_from(path.as_os_str().as_bytes().len()).ok()
}

#[cfg(not(any(windows, unix)))]
fn path_payload_bytes(path: &Path) -> Option<u64> {
    u64::try_from(path.to_string_lossy().len()).ok()
}
