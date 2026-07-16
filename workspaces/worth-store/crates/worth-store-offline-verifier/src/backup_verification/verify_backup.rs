use sha2::Digest;
use worth_store_physical_backend::OfflineMediaClosureEntry;
use worth_store_physical_format::{BackupBundleFormatAuthority, MaterializedBackupBundle};

use super::{
    BackupVerificationBudget, BackupVerificationReport, BackupVerificationReportEvidence,
    StructurallyVerifiedBackupBundle,
};

mod media_inspection;
mod owner_verification;
mod structural_comparison;
mod truth_completion;

use crate::backup_verification::verification_owned_memory::defect_owned_allocation_bytes;
use crate::inspection::reject_inspection_interruption;
use media_inspection::{admit_complete_backup_read_budget, inspect_backup_media};
use owner_verification::verify_backup_owner_semantics;
use structural_comparison::compare_backup_structure;
use truth_completion::{complete_backup_truth, BackupTruthCompletion};

#[derive(Debug)]
pub enum BackupStructuralVerificationDenial {
    Acquisition(crate::OfflineMediaAcquisitionDenial),
    Inspection(crate::OfflineInspectionDenial),
    Format(worth_store_physical_format::BackupBundleFormatDenial),
    OwnerBindingDuplicateSource,
    OwnerBindingMissingSource,
    PhysicalOwnershipOverlap,
    VerificationAllocationFailed,
    VerificationCounterOverflow,
    OwnedAllocationBudgetExceeded {
        phase: BackupVerificationAllocationPhase,
        admitted: u64,
        limit: u64,
    },
    TruthEvidence(crate::OfflineTruthEvidenceAdmissionDenial),
    TruthComposition(crate::OperationalTruthCompositionDenial),
    ConsistencyBasis(worth_store_physical_backend::OfflineMediaConsistencyBasisDenial),
    Defects(BackupVerificationReport),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupVerificationAllocationPhase {
    ManifestRetention,
    StructuralComparison,
    OwnerSemanticVerification,
    TruthEvidenceConstruction,
    TruthComposition,
}

pub fn verify_materialized_backup(
    materialized: MaterializedBackupBundle,
    budget: impl Into<BackupVerificationBudget>,
) -> Result<StructurallyVerifiedBackupBundle, BackupStructuralVerificationDenial> {
    verify_materialized_backup_with_cancellation(
        materialized,
        budget,
        crate::OfflineInspectionCancellation::new(),
    )
}

pub fn verify_materialized_backup_with_cancellation(
    materialized: MaterializedBackupBundle,
    budget: impl Into<BackupVerificationBudget>,
    cancellation: crate::OfflineInspectionCancellation,
) -> Result<StructurallyVerifiedBackupBundle, BackupStructuralVerificationDenial> {
    verify_materialized_backup_with_auxiliary_components(
        materialized,
        budget.into(),
        cancellation,
        &[],
    )
}

pub(crate) fn verify_staged_materialized_backup(
    materialized: MaterializedBackupBundle,
    budget: BackupVerificationBudget,
    staging_plan_fingerprint: [u8; 32],
    owner_effect_fingerprint: [u8; 32],
) -> Result<StructurallyVerifiedBackupBundle, BackupStructuralVerificationDenial> {
    let marker_digest = sha2::Sha256::digest(staging_plan_fingerprint).into();
    let mut closed_marker = [0_u8; 64];
    closed_marker[..32].copy_from_slice(&staging_plan_fingerprint);
    closed_marker[32..].copy_from_slice(&owner_effect_fingerprint);
    let auxiliary = [
        OfflineMediaClosureEntry::new(
            materialized.root().join(".closed-staging"),
            64,
            sha2::Sha256::digest(closed_marker).into(),
        )
        .expect("closed staging marker path is non-empty"),
        OfflineMediaClosureEntry::new(
            materialized.root().join(".staging-identity"),
            32,
            marker_digest,
        )
        .expect("staging identity marker path is non-empty"),
    ];
    verify_materialized_backup_with_auxiliary_components(
        materialized,
        budget,
        crate::OfflineInspectionCancellation::new(),
        &auxiliary,
    )
}

fn verify_materialized_backup_with_auxiliary_components(
    materialized: MaterializedBackupBundle,
    budget: BackupVerificationBudget,
    cancellation: crate::OfflineInspectionCancellation,
    admitted_auxiliary_components: &[OfflineMediaClosureEntry],
) -> Result<StructurallyVerifiedBackupBundle, BackupStructuralVerificationDenial> {
    let started_at = std::time::Instant::now();
    let inspection_budget = budget.inspection();
    reject_verification_interruption(inspection_budget, &cancellation, started_at)?;
    let current = BackupBundleFormatAuthority::canonical()
        .admit_materialized_with_limits(materialized.root(), budget.manifest())
        .map_err(BackupStructuralVerificationDenial::Format)?;
    reject_verification_interruption(inspection_budget, &cancellation, started_at)?;
    let manifest_read = current.manifest_read_observation();
    let admitted_read_bytes = admit_complete_backup_read_budget(
        &current,
        admitted_auxiliary_components,
        inspection_budget,
    )?;
    let walked = inspect_backup_media(
        &current,
        budget,
        cancellation.clone(),
        started_at,
        admitted_auxiliary_components,
    )?;
    reject_verification_interruption(inspection_budget, &cancellation, started_at)?;
    let structural_comparison = compare_backup_structure(
        &current,
        &materialized,
        &walked,
        admitted_auxiliary_components,
        inspection_budget,
        &cancellation,
        started_at,
    )?;
    let owner = verify_backup_owner_semantics(
        &current,
        &walked,
        inspection_budget,
        admitted_read_bytes,
        structural_comparison,
        cancellation.clone(),
        started_at,
    )?;
    if !owner.defects.is_empty() {
        return Err(BackupStructuralVerificationDenial::Defects(
            BackupVerificationReport::new(BackupVerificationReportEvidence {
                defects: owner.defects,
                admitted_read_bytes: owner.admitted_read_bytes,
                inspected_bytes: owner.observed_read_bytes,
                inspected_files: owner.inspected_files,
                peak_buffer_bytes: owner.peak_buffer_bytes,
                owner_verified_artifacts: owner.counters.artifacts_verified(),
                owner_verified_bytes: owner.counters.bytes_verified(),
                owner_decoder_allocation_bytes: owner.counters.decoder_allocation_bytes(),
                manifest_owned_allocation_bytes: manifest_read.owned_allocation_bytes(),
                peak_owned_allocation_bytes: owner.peak_owned_allocation_bytes,
                read_accounting: owner.read_accounting,
            }),
        ));
    }
    let defects_owned_allocation_bytes = defect_owned_allocation_bytes(&owner.defects)
        .ok_or(BackupStructuralVerificationDenial::VerificationCounterOverflow)?;
    let truth = complete_backup_truth(BackupTruthCompletion {
        current: &current,
        walked,
        defect_owned_allocation_bytes: defects_owned_allocation_bytes,
        owner_bindings: owner.owner_bindings,
        admitted_auxiliary_components,
        recovery_candidates: owner.recovery_candidates,
        inspection_budget,
        cancellation: &cancellation,
        started_at,
    })?;
    reject_verification_interruption(inspection_budget, &cancellation, started_at)?;
    let report = BackupVerificationReport::new(BackupVerificationReportEvidence {
        defects: owner.defects,
        admitted_read_bytes: owner.admitted_read_bytes,
        inspected_bytes: owner.observed_read_bytes,
        inspected_files: owner.inspected_files,
        peak_buffer_bytes: owner.peak_buffer_bytes,
        owner_verified_artifacts: owner.counters.artifacts_verified(),
        owner_verified_bytes: owner.counters.bytes_verified(),
        owner_decoder_allocation_bytes: owner.counters.decoder_allocation_bytes(),
        manifest_owned_allocation_bytes: manifest_read.owned_allocation_bytes(),
        peak_owned_allocation_bytes: owner
            .peak_owned_allocation_bytes
            .max(truth.peak_owned_allocation_bytes),
        read_accounting: owner.read_accounting,
    });
    Ok(StructurallyVerifiedBackupBundle::new(
        current,
        truth.verification_identity,
        report,
        truth.operational_truth,
    ))
}

fn reject_verification_interruption(
    budget: crate::OfflineInspectionBudget,
    cancellation: &crate::OfflineInspectionCancellation,
    started_at: std::time::Instant,
) -> Result<(), BackupStructuralVerificationDenial> {
    reject_inspection_interruption(budget, cancellation, started_at)
        .map_err(BackupStructuralVerificationDenial::Inspection)
}
