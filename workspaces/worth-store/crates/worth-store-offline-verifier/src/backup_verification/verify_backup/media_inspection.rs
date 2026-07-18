use worth_store_physical_backend::{OfflineMediaClosureEntry, OfflineMediaConsistencyBasis};
use worth_store_physical_format::MaterializedBackupBundle;

use crate::{OfflineStoreInspection, StructurallyWalkedMedia, UntrustedOfflineMediaSet};

use super::BackupStructuralVerificationDenial;
use crate::backup_verification::verification_support::{
    closure_defect_report, hex, invalid_manifest,
};
use crate::backup_verification::BackupVerificationBudget;

pub(super) fn inspect_backup_media(
    current: &MaterializedBackupBundle,
    budget: BackupVerificationBudget,
    cancellation: crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
    admitted_auxiliary_components: &[OfflineMediaClosureEntry],
) -> Result<StructurallyWalkedMedia, BackupStructuralVerificationDenial> {
    let inspection_budget = budget.inspection();
    let manifest_read = current.manifest_read_observation();
    let inspection_owned_limit = inspection_budget
        .maximum_owned_allocation_bytes()
        .checked_sub(manifest_read.owned_allocation_bytes())
        .ok_or(
            BackupStructuralVerificationDenial::OwnedAllocationBudgetExceeded {
                phase: super::BackupVerificationAllocationPhase::ManifestRetention,
                admitted: manifest_read.owned_allocation_bytes(),
                limit: inspection_budget.maximum_owned_allocation_bytes(),
            },
        )?;
    let media_inspection_budget = inspection_budget
        .with_maximum_owned_allocation_bytes(inspection_owned_limit)
        .ok_or(
            BackupStructuralVerificationDenial::OwnedAllocationBudgetExceeded {
                phase: super::BackupVerificationAllocationPhase::ManifestRetention,
                admitted: manifest_read
                    .owned_allocation_bytes()
                    .saturating_add(inspection_budget.max_buffer_bytes() as u64),
                limit: inspection_budget.maximum_owned_allocation_bytes(),
            },
        )?;
    let manifest_entry = OfflineMediaClosureEntry::new(
        current.root().join("backup.manifest"),
        std::fs::metadata(current.root().join("backup.manifest"))
            .map_err(|source| {
                BackupStructuralVerificationDenial::Format(
                    worth_store_physical_format::BackupBundleFormatDenial::Read(source),
                )
            })?
            .len(),
        current.manifest_digest(),
    )
    .ok_or_else(invalid_manifest)?;
    let mut component_entries = Vec::new();
    component_entries
        .try_reserve_exact(current.manifest().artifacts().len())
        .map_err(|_| BackupStructuralVerificationDenial::VerificationAllocationFailed)?;
    for row in current.manifest().artifacts() {
        crate::inspection::reject_inspection_interruption(
            inspection_budget,
            &cancellation,
            started_at,
        )
        .map_err(BackupStructuralVerificationDenial::Inspection)?;
        let path = current.root().join(row.output_name());
        match std::fs::symlink_metadata(&path) {
            Ok(_) => component_entries.push(
                OfflineMediaClosureEntry::new(path, row.bytes(), row.content_digest())
                    .ok_or_else(invalid_manifest)?,
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                // Absence is a structural defect, not a reason to hide every
                // other independently inspectable component. The immutable
                // basis covers the manifest and every component present at
                // acquisition; structural comparison records the missing row.
            }
            Err(error) => {
                return Err(BackupStructuralVerificationDenial::Format(
                    worth_store_physical_format::BackupBundleFormatDenial::Read(error),
                ));
            }
        }
    }
    let basis = OfflineMediaConsistencyBasis::content_addressed_closure(
        hex(&current.manifest_digest())?,
        std::iter::once(manifest_entry)
            .chain(component_entries)
            .chain(admitted_auxiliary_components.iter().cloned()),
    )
    .map_err(BackupStructuralVerificationDenial::ConsistencyBasis)?;
    let session = match OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        current.root(),
        basis,
    ))
    .budget(media_inspection_budget)
    .cancellation(cancellation)
    .start()
    {
        Ok(session) => session,
        Err(crate::OfflineMediaAcquisitionDenial::Interrupted(denial)) => {
            return Err(BackupStructuralVerificationDenial::Inspection(denial));
        }
        Err(crate::OfflineMediaAcquisitionDenial::Media(denial)) => {
            if let Some(report) = closure_defect_report(&denial, current.root()) {
                return Err(BackupStructuralVerificationDenial::Defects(report));
            }
            return Err(BackupStructuralVerificationDenial::Acquisition(
                crate::OfflineMediaAcquisitionDenial::Media(denial),
            ));
        }
        Err(denial) => return Err(BackupStructuralVerificationDenial::Acquisition(denial)),
    };
    match session.finish() {
        Ok(walked) => Ok(walked),
        Err(crate::OfflineInspectionDenial::Media(denial)) => {
            if let Some(report) = closure_defect_report(&denial, current.root()) {
                return Err(BackupStructuralVerificationDenial::Defects(report));
            }
            Err(BackupStructuralVerificationDenial::Inspection(
                crate::OfflineInspectionDenial::Media(denial),
            ))
        }
        Err(denial) => Err(BackupStructuralVerificationDenial::Inspection(denial)),
    }
}

pub(super) fn admit_complete_backup_read_budget(
    current: &MaterializedBackupBundle,
    admitted_auxiliary_components: &[OfflineMediaClosureEntry],
    budget: crate::OfflineInspectionBudget,
) -> Result<u64, BackupStructuralVerificationDenial> {
    let component_bytes = current
        .manifest()
        .artifacts()
        .iter()
        .try_fold(0_u64, |total, row| total.checked_add(row.bytes()))
        .and_then(|total| {
            admitted_auxiliary_components
                .iter()
                .try_fold(total, |sum, entry| sum.checked_add(entry.bytes()))
        })
        .ok_or_else(|| read_budget_denial(u64::MAX, budget))?;
    let admitted = current
        .manifest_read_observation()
        .encoded_bytes()
        .checked_add(component_bytes)
        .and_then(|one_complete_pass| one_complete_pass.checked_mul(2))
        .ok_or_else(|| read_budget_denial(u64::MAX, budget))?;
    if admitted > budget.max_total_read_bytes() {
        Err(read_budget_denial(admitted, budget))
    } else {
        Ok(admitted)
    }
}

fn read_budget_denial(
    admitted: u64,
    budget: crate::OfflineInspectionBudget,
) -> BackupStructuralVerificationDenial {
    BackupStructuralVerificationDenial::Inspection(
        crate::OfflineInspectionDenial::ReadBudgetExceeded {
            admitted,
            limit: budget.max_total_read_bytes(),
        },
    )
}
