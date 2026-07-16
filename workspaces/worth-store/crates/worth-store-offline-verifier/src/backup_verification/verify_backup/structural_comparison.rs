use worth_store_physical_format::MaterializedBackupBundle;

use crate::{OfflineInspectionBudget, StructurallyWalkedMedia};
use worth_store_physical_backend::OfflineMediaClosureEntry;

use super::{BackupStructuralVerificationDenial, BackupVerificationAllocationPhase};
use crate::backup_verification::verification_owned_memory::{
    defect_owned_allocation_bytes, maximum_structural_comparison_owned_allocation_bytes,
    structural_working_set_bytes,
};
use crate::backup_verification::BackupVerificationDefect;

pub(super) struct StructuralBackupComparison {
    pub(super) defects: Vec<BackupVerificationDefect>,
    pub(super) base_retained_owned_allocation_bytes: u64,
    pub(super) retained_owned_allocation_bytes: u64,
    pub(super) peak_owned_allocation_bytes: u64,
}

pub(super) fn compare_backup_structure(
    current: &MaterializedBackupBundle,
    originally_admitted: &MaterializedBackupBundle,
    walked: &StructurallyWalkedMedia,
    admitted_auxiliary_components: &[OfflineMediaClosureEntry],
    budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<StructuralBackupComparison, BackupStructuralVerificationDenial> {
    let base_retained_owned_allocation_bytes = current
        .manifest_read_observation()
        .owned_allocation_bytes()
        .checked_add(
            walked
                .owned_allocation_bytes()
                .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?,
        )
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    let defect_capacity = current
        .manifest()
        .artifacts()
        .len()
        .checked_mul(5)
        .and_then(|count| count.checked_add(20))
        .ok_or(BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    preflight_structural_comparison(
        current,
        walked,
        budget,
        base_retained_owned_allocation_bytes,
        defect_capacity,
    )?;

    let mut defects = Vec::new();
    defects
        .try_reserve_exact(defect_capacity)
        .map_err(|_| BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    if current.manifest_digest() != originally_admitted.manifest_digest()
        || current.manifest() != originally_admitted.manifest()
    {
        defects.push(BackupVerificationDefect::PublishedManifestChanged);
    }
    let validation_workspace =
        crate::backup_verification::manifest_semantic_validation::validate_manifest(
            current.manifest(),
            &mut defects,
        )
        .map_err(|_| BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    let validation_peak = base_retained_owned_allocation_bytes
        .checked_add(
            defect_owned_allocation_bytes(&defects)
                .and_then(|bytes| bytes.checked_add(validation_workspace))
                .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?,
        )
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;

    let mut walked_by_name = Vec::new();
    walked_by_name
        .try_reserve_exact(walked.files().len())
        .map_err(|_| BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    walked_by_name.extend(
        walked
            .files()
            .iter()
            .filter(|file| file.path().parent() == Some(current.root()))
            .filter_map(|file| {
                file.path()
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| (name, file))
            }),
    );
    walked_by_name.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let mut expected_names = Vec::new();
    expected_names
        .try_reserve_exact(current.manifest().artifacts().len())
        .map_err(|_| BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    expected_names.extend(
        current
            .manifest()
            .artifacts()
            .iter()
            .map(|row| row.output_name()),
    );
    expected_names.sort_unstable();
    compare_expected_components(
        current,
        &walked_by_name,
        &mut defects,
        budget,
        cancellation,
        started_at,
    )?;
    let expected_components = ExpectedBackupComponents {
        manifest_artifacts: &expected_names,
        admitted_auxiliary: admitted_auxiliary_components,
    };
    reject_unexpected_components(
        current,
        walked,
        expected_components,
        &mut defects,
        budget,
        cancellation,
        started_at,
    )?;

    let comparison_working_set = structural_working_set_bytes(
        &defects,
        walked_by_name.capacity(),
        expected_names.capacity(),
    )
    .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    let comparison_peak = base_retained_owned_allocation_bytes
        .checked_add(comparison_working_set)
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    let retained_owned_allocation_bytes = base_retained_owned_allocation_bytes
        .checked_add(
            defect_owned_allocation_bytes(&defects)
                .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?,
        )
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    Ok(StructuralBackupComparison {
        defects,
        base_retained_owned_allocation_bytes,
        retained_owned_allocation_bytes,
        peak_owned_allocation_bytes: validation_peak.max(comparison_peak),
    })
}

fn preflight_structural_comparison(
    current: &MaterializedBackupBundle,
    walked: &StructurallyWalkedMedia,
    budget: OfflineInspectionBudget,
    base_retained_owned_allocation_bytes: u64,
    defect_capacity: usize,
) -> Result<(), BackupStructuralVerificationDenial> {
    let maximum_workspace = maximum_structural_comparison_owned_allocation_bytes(
        current.manifest(),
        walked,
        defect_capacity,
    )
    .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    let admitted = base_retained_owned_allocation_bytes
        .checked_add(maximum_workspace)
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    if admitted > budget.maximum_owned_allocation_bytes() {
        Err(
            BackupStructuralVerificationDenial::OwnedAllocationBudgetExceeded {
                phase: BackupVerificationAllocationPhase::StructuralComparison,
                admitted,
                limit: budget.maximum_owned_allocation_bytes(),
            },
        )
    } else {
        Ok(())
    }
}

fn compare_expected_components(
    current: &MaterializedBackupBundle,
    walked_by_name: &[(&str, &crate::OfflineWalkedFile)],
    defects: &mut Vec<BackupVerificationDefect>,
    budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<(), BackupStructuralVerificationDenial> {
    for row in current.manifest().artifacts() {
        reject_interruption(budget, cancellation, started_at)?;
        let Some(file) = walked_by_name
            .binary_search_by(|entry| entry.0.cmp(row.output_name()))
            .ok()
            .map(|index| walked_by_name[index].1)
        else {
            defects.push(BackupVerificationDefect::MissingComponent {
                output_name: row.output_name().to_owned(),
            });
            continue;
        };
        if file.length() != row.bytes() {
            defects.push(BackupVerificationDefect::ComponentLengthMismatch {
                output_name: row.output_name().to_owned(),
                expected: row.bytes(),
                actual: file.length(),
            });
        }
        if file.content_digest() != row.content_digest() {
            defects.push(BackupVerificationDefect::ComponentDigestMismatch {
                output_name: row.output_name().to_owned(),
            });
        }
    }
    Ok(())
}

struct ExpectedBackupComponents<'a> {
    manifest_artifacts: &'a [&'a str],
    admitted_auxiliary: &'a [OfflineMediaClosureEntry],
}

fn reject_unexpected_components(
    current: &MaterializedBackupBundle,
    walked: &StructurallyWalkedMedia,
    expected: ExpectedBackupComponents<'_>,
    defects: &mut Vec<BackupVerificationDefect>,
    budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<(), BackupStructuralVerificationDenial> {
    for file in walked.files() {
        reject_interruption(budget, cancellation, started_at)?;
        let Some(name) = file.path().file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let direct_child = file.path().parent() == Some(current.root());
        if !direct_child
            || (name != "backup.manifest"
                && expected.manifest_artifacts.binary_search(&name).is_err()
                && !expected.admitted_auxiliary.iter().any(|entry| {
                    entry.path().file_name().and_then(|value| value.to_str()) == Some(name)
                }))
        {
            defects.push(BackupVerificationDefect::ExtraComponent {
                path: file.path().to_path_buf(),
            });
        }
    }
    Ok(())
}

fn reject_interruption(
    budget: OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<(), BackupStructuralVerificationDenial> {
    crate::inspection::reject_inspection_interruption(budget, cancellation, started_at)
        .map_err(BackupStructuralVerificationDenial::Inspection)
}
