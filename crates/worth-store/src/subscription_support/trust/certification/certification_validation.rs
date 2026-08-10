use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::batch_scope::SupportCertificationBatchScope;
use super::counter_snapshot::SupportCertificationCounterSnapshot;
use super::coverage_matrix::SupportCertificationCoverageMatrix;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(super) fn validate_certification_counters(
    matrix: &SupportCertificationCoverageMatrix,
    batch_scope: SupportCertificationBatchScope,
    counter_snapshot: SupportCertificationCounterSnapshot,
) -> Result<(), SupportTrustFailure> {
    let row_count = matrix.rows().len() as u64;
    let first_ship_family_count = matrix
        .rows()
        .iter()
        .map(|row| row.evidence().family_kind)
        .collect::<BTreeSet<_>>()
        .len() as u64;
    if batch_scope.row_count() != row_count
        || counter_snapshot.coverage_row_count() != row_count
        || counter_snapshot.first_ship_family_count() != first_ship_family_count
        || counter_snapshot.receipt_reuse_count() != batch_scope.expected_receipt_reuse_count()
        || counter_snapshot.index_probe_count() != batch_scope.expected_index_probes()
        || counter_snapshot.allocation_count() != batch_scope.expected_allocation_count()
        || counter_snapshot.forbidden_exact_overclaim_count() != 0
        || counter_snapshot.global_scan_debt_count() != 0
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "support trust certification bundle counters must match declared scope and zero-overclaim invariants",
        ));
    }
    Ok(())
}

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<String, SupportTrustFailure> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "support trust certification evidence must serialize deterministically",
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

pub(super) fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("support trust certification {label} must be non-empty"),
        ));
    }
    Ok(value)
}
