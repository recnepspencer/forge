use crate::{
    authority::AuthoritativeExportBundle,
    evidence::StoreCounterSnapshot,
    recovery::{
        BackupRestoreCompatibilityReport, DegradedStateReport, MaintenanceRecoveryReport,
        RecoverySourceReport, RecoveryStatusReport, SupportArtifactRecoveryReport,
    },
};
use serde::Serialize;

use super::common::{
    CompatibilityDigestBasis, ObservedRecoveryFailure356, QuiescenceReport,
    RecoveryCertificationSummary, stable_digest,
};

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
        let compatibility_digest =
            stable_digest(&CompatibilityDigestBasis { backup_restore_compatibility_report: &backup_restore_compatibility_report });
        let quiescence_report = QuiescenceReport {
            planned_mutation_count: recovery_status_report.planned_mutation_count(),
            recovered_decision_count: recovery_status_report.recovered_decision_count(),
            quiescent_restart: recovery_status_report.quiescent_restart(),
            recovery_quiescent_restart_count: counter_snapshot.recovery_quiescent_restart_count,
            recovery_non_quiescent_restart_count: counter_snapshot.recovery_non_quiescent_restart_count,
        };
        let certification_summary = RecoveryCertificationSummary {
            source_report_count: recovery_source_report.len(),
            fallback_source_count: recovery_source_report
                .iter()
                .filter(|report| !matches!(report.source_kind(), crate::RecoverySourceKind::PublishedAuthoritativeTruth))
                .count(),
            quarantine_source_count: recovery_source_report
                .iter()
                .filter(|report| matches!(report.source_kind(), crate::RecoverySourceKind::RequiresQuarantine))
                .count(),
            degraded_quarantine_count: degraded_state_report.quarantines().len(),
            degraded_retained_without_ack_count: degraded_state_report.retained_without_acknowledgment().len(),
            degraded_rebuild_required_count: degraded_state_report.rebuilds().len(),
            maintenance_rebuild_required_count: maintenance_recovery_report
                .entries()
                .iter()
                .filter(|entry| matches!(entry.disposition(), crate::MaintenanceRecoveryDisposition::RequireRebuild))
                .count(),
            support_artifact_rebuild_required_count: support_artifact_recovery_report.rebuilds().len(),
            support_artifact_quarantine_required_count: support_artifact_recovery_report.quarantines().len(),
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
