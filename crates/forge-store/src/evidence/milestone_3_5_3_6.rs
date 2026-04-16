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
pub struct Milestone35CertificationBundle {
    pub artifact_digest: String,
    pub write_path_digest: String,
    pub ack_boundary_report: PublicationWriteOutcome,
    pub certification_summary: WritePathCertificationSummary,
    pub media_barrier_matrix: MediaBarrierMatrix,
    pub tail_validation_report: TailValidationReport,
    pub observed_failures: Vec<ObservedPublicationFailure>,
    pub failure_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone36CertificationBundle {
    pub truth_digest: String,
    pub artifact_digest: String,
    pub recovery_source_report: Vec<RecoverySourceReport>,
    pub maintenance_recovery_report: MaintenanceRecoveryReport,
    pub support_artifact_recovery_report: SupportArtifactRecoveryReport,
    pub degraded_state_report: DegradedStateReport,
    pub certification_summary: RecoveryCertificationSummary,
    pub backup_restore_compatibility_report: BackupRestoreCompatibilityReport,
    pub compatibility_digest: String,
    pub quiescence_report: QuiescenceReport,
    pub recovery_status_report: RecoveryStatusReport,
    pub observed_failures: Vec<ObservedRecoveryFailure356>,
    pub failure_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
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

impl Milestone35CertificationBundle {
    pub fn new(
        media_report: DurableMediaReport,
        ack_boundary_report: PublicationWriteOutcome,
        counter_snapshot: StoreCounterSnapshot,
        failures: &[ObservedPublicationFailure],
    ) -> Self {
        let media_barrier_matrix = MediaBarrierMatrix {
            backend_family: media_report.backend_family(),
            content_barrier: media_report.content_barrier(),
            metadata_barrier: media_report.metadata_barrier(),
            ack_required_barrier: media_report.ack_required_barrier(),
            family_states: ack_boundary_report.family_states().to_vec(),
        };
        let certification_summary = WritePathCertificationSummary {
            family_count: ack_boundary_report.family_states().len(),
            published_family_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| matches!(state.state(), crate::PublicationState::Published))
                .count(),
            publication_gap_family_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| matches!(state.state(), crate::PublicationState::PublicationGap))
                .count(),
            non_source_admitted_family_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| !state.source_admitted())
                .count(),
            barrier_complete_not_published_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| {
                    matches!(
                        state.state(),
                        crate::PublicationState::BarrierCompleteButNotPublished
                    )
                })
                .count(),
            sufficient_for_published_truth: ack_boundary_report.sufficient_for_published_truth(),
            acknowledgment_eligible: ack_boundary_report.acknowledgment_eligible(),
        };
        let tail_validation_report = TailValidationReport {
            durable_frame_scan_count: counter_snapshot.durable_frame_scan_count,
            durable_frame_reject_count: counter_snapshot.durable_frame_reject_count,
            durable_truncated_tail_count: counter_snapshot.durable_truncated_tail_count,
            durable_torn_write_count: counter_snapshot.durable_torn_write_count,
        };
        let artifact_digest = stable_digest(
            &ack_boundary_report
                .family_states()
                .iter()
                .map(|state| (state.family(), state.state(), state.source_admitted()))
                .collect::<Vec<_>>(),
        );
        let write_path_digest = stable_digest(&WritePathDigestBasis {
            media_report,
            ack_boundary_report: &ack_boundary_report,
            media_barrier_matrix: &media_barrier_matrix,
            tail_validation_report: &tail_validation_report,
        });

        Self {
            artifact_digest,
            write_path_digest,
            ack_boundary_report,
            certification_summary,
            media_barrier_matrix,
            tail_validation_report,
            observed_failures: failures.to_vec(),
            failure_digest: stable_digest(failures),
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 3.5 certification serialization")
    }
}

impl Milestone36CertificationBundle {
    pub fn new(
        recovered_export: &AuthoritativeExportBundle,
        recovery_status_report: RecoveryStatusReport,
        recovery_source_report: Vec<RecoverySourceReport>,
        maintenance_recovery_report: MaintenanceRecoveryReport,
        support_artifact_recovery_report: SupportArtifactRecoveryReport,
        degraded_state_report: DegradedStateReport,
        backup_restore_compatibility_report: BackupRestoreCompatibilityReport,
        counter_snapshot: StoreCounterSnapshot,
        failures: &[ObservedRecoveryFailure356],
    ) -> Self {
        let truth_digest = stable_digest(recovered_export);
        let artifact_digest = stable_digest(
            &recovered_export
                .clone()
                .into_canonicalized()
                .authoritative_artifact_digests,
        );
        let compatibility_digest = stable_digest(&CompatibilityDigestBasis {
            backup_restore_compatibility_report: &backup_restore_compatibility_report,
        });
        let quiescence_report = QuiescenceReport {
            planned_mutation_count: recovery_status_report.planned_mutation_count(),
            recovered_decision_count: recovery_status_report.recovered_decision_count(),
            quiescent_restart: recovery_status_report.quiescent_restart(),
            recovery_quiescent_restart_count: counter_snapshot.recovery_quiescent_restart_count,
            recovery_non_quiescent_restart_count: counter_snapshot
                .recovery_non_quiescent_restart_count,
        };
        let certification_summary = RecoveryCertificationSummary {
            source_report_count: recovery_source_report.len(),
            fallback_source_count: recovery_source_report
                .iter()
                .filter(|report| {
                    !matches!(
                        report.source_kind(),
                        crate::RecoverySourceKind::PublishedAuthoritativeTruth
                    )
                })
                .count(),
            quarantine_source_count: recovery_source_report
                .iter()
                .filter(|report| {
                    matches!(
                        report.source_kind(),
                        crate::RecoverySourceKind::RequiresQuarantine
                    )
                })
                .count(),
            degraded_quarantine_count: degraded_state_report.quarantines().len(),
            degraded_retained_without_ack_count: degraded_state_report
                .retained_without_acknowledgment()
                .len(),
            degraded_rebuild_required_count: degraded_state_report.rebuilds().len(),
            maintenance_rebuild_required_count: maintenance_recovery_report
                .entries()
                .iter()
                .filter(|entry| {
                    matches!(
                        entry.disposition(),
                        crate::MaintenanceRecoveryDisposition::RequireRebuild
                    )
                })
                .count(),
            support_artifact_rebuild_required_count: support_artifact_recovery_report
                .rebuilds()
                .len(),
            support_artifact_quarantine_required_count: support_artifact_recovery_report
                .quarantines()
                .len(),
            recommended_action_count: recovery_status_report.recommended_actions().len(),
        };

        Self {
            truth_digest,
            artifact_digest,
            recovery_source_report,
            maintenance_recovery_report,
            support_artifact_recovery_report,
            degraded_state_report,
            certification_summary,
            backup_restore_compatibility_report,
            compatibility_digest,
            quiescence_report,
            recovery_status_report,
            observed_failures: failures.to_vec(),
            failure_digest: stable_digest(failures),
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 3.6 certification serialization")
    }
}

#[derive(Serialize)]
struct WritePathDigestBasis<'a> {
    media_report: DurableMediaReport,
    ack_boundary_report: &'a PublicationWriteOutcome,
    media_barrier_matrix: &'a MediaBarrierMatrix,
    tail_validation_report: &'a TailValidationReport,
}

#[derive(Serialize)]
struct CompatibilityDigestBasis<'a> {
    backup_restore_compatibility_report: &'a BackupRestoreCompatibilityReport,
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("milestone 3.5/3.6 digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
