use crate::{
    authority::AuthoritativeExportBundle,
    evidence::StoreCounterSnapshot,
    failure::StoreErrorKind,
    publication::{ObservedPublicationFamilyState, PublicationWriteOutcome},
    recovery::{
        BackupRestoreCompatibilityReport, DegradedStateReport, MaintenanceRecoveryReport,
        RecoverySourceReport, RecoveryStatusReport, SupportArtifactRecoveryReport,
    },
    DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObservedPublicationFailure {
    pub kind: StoreErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObservedRecoveryFailure356 {
    pub kind: StoreErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MediaBarrierMatrix {
    pub backend_family: DurableBackendFamily,
    pub content_barrier: DurabilityBarrierClass,
    pub metadata_barrier: DurabilityBarrierClass,
    pub ack_required_barrier: DurabilityBarrierClass,
    pub family_states: Vec<ObservedPublicationFamilyState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TailValidationReport {
    pub durable_frame_scan_count: u64,
    pub durable_frame_reject_count: u64,
    pub durable_truncated_tail_count: u64,
    pub durable_torn_write_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WritePathCertificationSummary {
    pub family_count: usize,
    pub published_family_count: usize,
    pub publication_gap_family_count: usize,
    pub non_source_admitted_family_count: usize,
    pub barrier_complete_not_published_count: usize,
    pub sufficient_for_published_truth: bool,
    pub acknowledgment_eligible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuiescenceReport {
    pub planned_mutation_count: usize,
    pub recovered_decision_count: usize,
    pub quiescent_restart: bool,
    pub recovery_quiescent_restart_count: u64,
    pub recovery_non_quiescent_restart_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoveryCertificationSummary {
    pub source_report_count: usize,
    pub fallback_source_count: usize,
    pub quarantine_source_count: usize,
    pub degraded_quarantine_count: usize,
    pub degraded_retained_without_ack_count: usize,
    pub degraded_rebuild_required_count: usize,
    pub maintenance_rebuild_required_count: usize,
    pub support_artifact_rebuild_required_count: usize,
    pub support_artifact_quarantine_required_count: usize,
    pub recommended_action_count: usize,
}

impl ObservedPublicationFailure {
    pub fn from_error(error: &crate::StoreError) -> Self {
        Self {
            kind: error.kind().clone(),
            message: error.message().to_string(),
        }
    }
}

impl ObservedRecoveryFailure356 {
    pub fn from_error(error: &crate::StoreError) -> Self {
        Self {
            kind: error.kind().clone(),
            message: error.message().to_string(),
        }
    }
}

#[derive(Serialize)]
pub(super) struct WritePathDigestBasis<'a> {
    pub media_report: DurableMediaReport,
    pub ack_boundary_report: &'a PublicationWriteOutcome,
    pub media_barrier_matrix: &'a MediaBarrierMatrix,
    pub tail_validation_report: &'a TailValidationReport,
}

#[derive(Serialize)]
pub(super) struct CompatibilityDigestBasis<'a> {
    pub backup_restore_compatibility_report: &'a BackupRestoreCompatibilityReport,
}

pub(super) fn stable_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("milestone 3.5/3.6 digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[allow(dead_code)]
pub(super) fn _keep_imports_live(
    _: &AuthoritativeExportBundle,
    _: &StoreCounterSnapshot,
    _: &RecoverySourceReport,
    _: &MaintenanceRecoveryReport,
    _: &SupportArtifactRecoveryReport,
    _: &DegradedStateReport,
    _: &RecoveryStatusReport,
) {
}
