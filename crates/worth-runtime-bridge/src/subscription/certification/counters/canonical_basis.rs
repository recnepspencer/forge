use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeSubscriptionCertificationCounterSnapshot;

impl BridgeSubscriptionCertificationCounterSnapshot {
    pub fn canonical_basis(&self) -> Arc<str> {
        Arc::from(format!(
            concat!(
                "bridge-subscription-certification-counters|assembly-plan:{}|",
                "cost-profile:{}|bundle:{}|source-index-entries:{}|source-index-scans:{}|",
                "global-history-scans:{}|global-subscription-scans:{}|dense-rebuild:{}|",
                "over-budget-rejections:{}|scratch-allocations:{}|scratch-reuses:{}|",
                "comparison-plans:{}|bundle-comparisons:{}|comparison-mismatches:{}|",
                "failure-localizations:{}|offline-audit-indexes:{}|offline-audit-plans:{}|",
                "offline-audit-reports:{}|offline-audit-bundles:{}|",
                "offline-audit-comparison-reports:{}|host-log-dependencies:{}|",
                "live-state-dependencies:{}|reference-workload-lanes:{}|",
                "reference-workload-reports:{}|reference-workload-coverage-reports:{}|",
                "cost-posture-reports:{}|schema-parity-reports:{}|",
                "multi-failure-precedence-reports:{}|ordering-hostility-reports:{}|",
                "stale-checkpoint-reports:{}|bundle-insufficiency-reports:{}|",
                "historical-basis-reports:{}|strategy-lowering-reports:{}|",
                "fanout-reports:{}|denied-continuation-reports:{}|",
                "phase-18-support-matrix:{}|phase-18-closeout-artifacts:{}"
            ),
            self.bundle_assembly_plan_count,
            self.bundle_cost_profile_count,
            self.certification_bundle_count,
            self.source_artifact_index_entry_count,
            self.source_artifact_index_scan_count,
            self.global_history_scan_count,
            self.global_subscription_scan_count,
            self.dense_rebuild_count,
            self.over_budget_rejection_count,
            self.scratch_allocation_count,
            self.scratch_reuse_count,
            self.comparison_plan_count,
            self.bundle_comparison_count,
            self.bundle_comparison_mismatch_count,
            self.failure_localization_count,
            self.offline_audit_bundle_index_count,
            self.offline_audit_plan_count,
            self.offline_audit_report_count,
            self.offline_audit_bundle_count,
            self.offline_audit_comparison_report_count,
            self.host_log_dependency_count,
            self.live_state_dependency_count,
            self.reference_workload_lane_count,
            self.reference_workload_report_count,
            self.reference_workload_coverage_report_count,
            self.cost_posture_report_count,
            self.schema_parity_report_count,
            self.multi_failure_precedence_report_count,
            self.ordering_hostility_report_count,
            self.stale_checkpoint_report_count,
            self.bundle_insufficiency_report_count,
            self.historical_basis_report_count,
            self.strategy_lowering_report_count,
            self.fanout_report_count,
            self.denied_continuation_report_count,
            self.phase_18_support_matrix_count,
            self.phase_18_closeout_artifact_count,
        ))
    }

    pub fn digest(&self) -> Arc<str> {
        let canonical_basis = self.canonical_basis();
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Arc::from(format!(
            "bridge-subscription-certification-counters:sha256:{digest:x}"
        ))
    }
}
