use serde::Serialize;

use super::super::{
    DegradedStateReport, DurableRecoveryOutcome, DurableRecoveryPlan, MaintenanceRecoveryReport,
    RecoverySourceKind, SupportArtifactRecoveryReport,
};
use super::{
    actions::{
        build_recommended_actions, determine_operator_disposition, RecoveryOperatorAction,
        RecoveryOperatorDisposition,
    },
    bulk::{BulkRecoverySummary, RecoveredBulkChunk},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DurableRecoverySourceSummary {
    published_authoritative_truth: usize,
    hosted_runtime_canonical_result: usize,
    intent_only: usize,
    requires_rebuild: usize,
    requires_quarantine: usize,
    maintenance_residue: usize,
}

impl DurableRecoverySourceSummary {
    pub(crate) fn from_outcome(outcome: &DurableRecoveryOutcome) -> Self {
        let mut summary = Self {
            published_authoritative_truth: 0,
            hosted_runtime_canonical_result: 0,
            intent_only: 0,
            requires_rebuild: 0,
            requires_quarantine: 0,
            maintenance_residue: 0,
        };

        for report in &outcome.source_reports {
            match report.source_kind() {
                RecoverySourceKind::PublishedAuthoritativeTruth => {
                    summary.published_authoritative_truth += 1;
                }
                RecoverySourceKind::HostedRuntimeCanonicalResult => {
                    summary.hosted_runtime_canonical_result += 1;
                }
                RecoverySourceKind::IntentOnly => {
                    summary.intent_only += 1;
                }
                RecoverySourceKind::RequiresRebuild => {
                    summary.requires_rebuild += 1;
                }
                RecoverySourceKind::RequiresQuarantine => {
                    summary.requires_quarantine += 1;
                }
                RecoverySourceKind::MaintenanceResidue => {
                    summary.maintenance_residue += 1;
                }
            }
        }

        summary
    }

    pub fn published_authoritative_truth(&self) -> usize {
        self.published_authoritative_truth
    }
    pub fn hosted_runtime_canonical_result(&self) -> usize {
        self.hosted_runtime_canonical_result
    }
    pub fn intent_only(&self) -> usize {
        self.intent_only
    }
    pub fn requires_rebuild(&self) -> usize {
        self.requires_rebuild
    }
    pub fn requires_quarantine(&self) -> usize {
        self.requires_quarantine
    }
    pub fn maintenance_residue(&self) -> usize {
        self.maintenance_residue
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoveryStatusReport {
    planned_mutation_count: usize,
    recovered_decision_count: usize,
    quiescent_restart: bool,
    operator_disposition: RecoveryOperatorDisposition,
    source_summary: DurableRecoverySourceSummary,
    bulk_summary: BulkRecoverySummary,
    bulk_chunks: Vec<RecoveredBulkChunk>,
    degraded: DegradedStateReport,
    maintenance: MaintenanceRecoveryReport,
    support_artifacts: SupportArtifactRecoveryReport,
    recommended_actions: Vec<RecoveryOperatorAction>,
}

impl RecoveryStatusReport {
    pub(crate) fn new(
        plan: &DurableRecoveryPlan,
        outcome: &DurableRecoveryOutcome,
        maintenance: MaintenanceRecoveryReport,
        support_artifacts: SupportArtifactRecoveryReport,
    ) -> Self {
        let degraded = outcome.degraded_state_report();
        let source_summary = DurableRecoverySourceSummary::from_outcome(outcome);
        let bulk_chunks = RecoveredBulkChunk::collect_from_outcome(outcome);
        let bulk_summary = BulkRecoverySummary::from_bulk_chunks(&bulk_chunks);
        let operator_disposition =
            determine_operator_disposition(&degraded, &maintenance, &support_artifacts);
        let recommended_actions =
            build_recommended_actions(outcome, &degraded, &maintenance, &support_artifacts);

        Self {
            planned_mutation_count: plan.pending_durable_mutation_ids.len(),
            recovered_decision_count: outcome.decisions.len(),
            quiescent_restart: outcome.decisions.is_empty(),
            operator_disposition,
            source_summary,
            bulk_summary,
            bulk_chunks,
            degraded,
            maintenance,
            support_artifacts,
            recommended_actions,
        }
    }

    pub fn planned_mutation_count(&self) -> usize {
        self.planned_mutation_count
    }
    pub fn recovered_decision_count(&self) -> usize {
        self.recovered_decision_count
    }
    pub fn quiescent_restart(&self) -> bool {
        self.quiescent_restart
    }
    pub fn operator_disposition(&self) -> RecoveryOperatorDisposition {
        self.operator_disposition
    }
    pub fn source_summary(&self) -> &DurableRecoverySourceSummary {
        &self.source_summary
    }
    pub fn bulk_summary(&self) -> &BulkRecoverySummary {
        &self.bulk_summary
    }
    pub fn bulk_chunks(&self) -> &[RecoveredBulkChunk] {
        &self.bulk_chunks
    }
    pub fn degraded(&self) -> &DegradedStateReport {
        &self.degraded
    }
    pub fn maintenance(&self) -> &MaintenanceRecoveryReport {
        &self.maintenance
    }
    pub fn support_artifacts(&self) -> &SupportArtifactRecoveryReport {
        &self.support_artifacts
    }
    pub fn recommended_actions(&self) -> &[RecoveryOperatorAction] {
        &self.recommended_actions
    }
}
