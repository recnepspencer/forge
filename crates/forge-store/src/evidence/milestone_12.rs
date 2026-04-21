use std::collections::BTreeSet;

use crate::compatibility::{
    CompatibilityAdmissionCounters, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection,
    Milestone12CertificationRunSummary, Milestone12CompatibilityMatrix,
};
use serde::Serialize;

pub const MILESTONE_12_COUNTER_NAMES: &[&str] = &[
    "compatibility.admission.accepted_count",
    "compatibility.admission.rejected_count",
    "compatibility.registry.family_declaration_count",
    "compatibility.manifest.index_rebuild_count",
    "compatibility.manifest.entries_visited",
    "compatibility.manifest.index_lookup_count",
    "compatibility.manifest.digest_check_count",
    "compatibility.manifest.digest_recompute_count",
    "compatibility.manifest.publication_count",
    "compatibility.manifest.recovery_record_count",
    "compatibility.manifest.publication_gap_count",
    "compatibility.manifest.digest_mismatch_count",
    "compatibility.manifest.window_mismatch_count",
    "compatibility.receipt.reuse_rejection_count",
    "compatibility.receipt.reuse_hit_count",
    "compatibility.receipt.basis_mismatch_count",
    "compatibility.relation.recheck_count",
    "compatibility.edge.missing_rejection_count",
    "compatibility.index.row_scan_count",
    "compatibility.decode.malformed_frame_count",
    "compatibility.adapter.cost_class_count",
    "compatibility.adapter.parity_failure_count",
    "compatibility.adapter.hot_path_rejection_count",
    "compatibility.adapter.maintenance_required_rejection_count",
    "compatibility.adapter.out_of_scope_rejection_count",
    "compatibility.admission.native_count",
    "compatibility.admission.forward_backward_count",
    "compatibility.admission.adapter_count",
    "compatibility.authoritative.partial_truth_rejection_count",
    "compatibility.derived.reuse_incompatibility_count",
    "compatibility.derived.rebuild_incompatibility_count",
    "compatibility.derived.rebuild_required_count",
    "compatibility.derived.invalidation_count",
    "compatibility.derived.stale_version_rejection_count",
    "compatibility.derived.rebuild_debt_count",
    "compatibility.maintenance.rebuild_admission_count",
    "compatibility.maintenance.rebuild_rejection_count",
    "compatibility.derived.lane_plan_count",
    "compatibility.derived.lane_reuse_count",
    "compatibility.derived.lane_invalidation_count",
    "compatibility.derived.lane_rejection_count",
    "compatibility.derived.snapshot_reuse_count",
    "compatibility.derived.delta_reuse_count",
    "compatibility.derived.layout_basis_rejection_count",
    "compatibility.derived.bulk_resume_rejection_count",
    "compatibility.derived.maintenance_summary_rebuild_count",
    "compatibility.tier.non_authority_preserved_count",
    "compatibility.tier.manifest_rejection_count",
    "compatibility.maintenance.lane_mismatch_rejection_count",
    "compatibility.restore.out_of_scope_scan_count",
    "compatibility.restore.accept_count",
    "compatibility.restore.rejection_count",
    "compatibility.restore.publication_conflict_rejection_count",
    "compatibility.disaster_recovery.truth_window_count",
    "compatibility.disaster_recovery.derived_window_count",
    "compatibility.rolling.window_admission_count",
    "compatibility.rolling.window_rejection_count",
    "compatibility.rolling.multi_writer_rejection_count",
    "compatibility.rolling.mixed_version_skew_count",
];

pub const MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES: &[&str] = &[
    "compatibility.admission.accepted_count",
    "compatibility.admission.rejected_count",
    "compatibility.relation.recheck_count",
    "compatibility.edge.missing_rejection_count",
    "compatibility.receipt.reuse_hit_count",
    "compatibility.manifest.index_rebuild_count",
    "compatibility.manifest.entries_visited",
    "compatibility.manifest.index_lookup_count",
    "compatibility.manifest.digest_check_count",
    "compatibility.manifest.publication_count",
    "compatibility.manifest.recovery_record_count",
    "compatibility.manifest.publication_gap_count",
    "compatibility.manifest.digest_mismatch_count",
    "compatibility.manifest.window_mismatch_count",
    "compatibility.receipt.basis_mismatch_count",
    "compatibility.index.row_scan_count",
    "compatibility.decode.malformed_frame_count",
    "compatibility.adapter.hot_path_rejection_count",
    "compatibility.adapter.maintenance_required_rejection_count",
    "compatibility.adapter.out_of_scope_rejection_count",
    "compatibility.admission.native_count",
    "compatibility.admission.forward_backward_count",
    "compatibility.admission.adapter_count",
    "compatibility.authoritative.partial_truth_rejection_count",
    "compatibility.derived.reuse_incompatibility_count",
    "compatibility.derived.rebuild_incompatibility_count",
    "compatibility.derived.rebuild_required_count",
    "compatibility.derived.invalidation_count",
    "compatibility.derived.stale_version_rejection_count",
    "compatibility.derived.rebuild_debt_count",
    "compatibility.maintenance.rebuild_admission_count",
    "compatibility.maintenance.rebuild_rejection_count",
    "compatibility.derived.lane_plan_count",
    "compatibility.derived.lane_reuse_count",
    "compatibility.derived.lane_invalidation_count",
    "compatibility.derived.lane_rejection_count",
    "compatibility.derived.snapshot_reuse_count",
    "compatibility.derived.delta_reuse_count",
    "compatibility.derived.layout_basis_rejection_count",
    "compatibility.derived.bulk_resume_rejection_count",
    "compatibility.derived.maintenance_summary_rebuild_count",
    "compatibility.tier.non_authority_preserved_count",
    "compatibility.tier.manifest_rejection_count",
    "compatibility.maintenance.lane_mismatch_rejection_count",
    "compatibility.rolling.window_admission_count",
    "compatibility.rolling.window_rejection_count",
    "compatibility.rolling.multi_writer_rejection_count",
    "compatibility.rolling.mixed_version_skew_count",
    "compatibility.restore.accept_count",
    "compatibility.restore.rejection_count",
    "compatibility.restore.out_of_scope_scan_count",
    "compatibility.restore.publication_conflict_rejection_count",
    "compatibility.disaster_recovery.truth_window_count",
    "compatibility.disaster_recovery.derived_window_count",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CompatibilityMatrixRow {
    AuthoritativeRead,
    AuthoritativeWrite,
    DerivedReuse,
    DerivedRebuild,
    DerivedSnapshotReuseAccepted,
    DerivedDeltaReuseAccepted,
    LayoutBasisSkewRejected,
    BulkResumeSkewRejected,
    MaintenanceSummaryRebuildAdmitted,
    TierManifestNonAuthorityPreserved,
    TierManifestSkewRejected,
    Restore,
    RestoreScopedBackupAdmitted,
    RestoreOutOfScopeRejected,
    RestorePublicationConflictRejected,
    RestoreMissingEdgeRejected,
    RollingUpgrade,
    RollingUpgradeTwoCapabilityAdmitted,
    RollingUpgradeMultiWriterRejected,
    RollingUpgradeAdapterRejected,
    AdapterParity,
}

impl Milestone12CompatibilityMatrixRow {
    pub const fn label(self) -> &'static str {
        match self {
            Self::AuthoritativeRead => "authoritative_read",
            Self::AuthoritativeWrite => "authoritative_write",
            Self::DerivedReuse => "derived_reuse",
            Self::DerivedRebuild => "derived_rebuild",
            Self::DerivedSnapshotReuseAccepted => "derived_snapshot_reuse_accepted",
            Self::DerivedDeltaReuseAccepted => "derived_delta_reuse_accepted",
            Self::LayoutBasisSkewRejected => "layout_basis_skew_rejected",
            Self::BulkResumeSkewRejected => "bulk_resume_skew_rejected",
            Self::MaintenanceSummaryRebuildAdmitted => "maintenance_summary_rebuild_admitted",
            Self::TierManifestNonAuthorityPreserved => "tier_manifest_non_authority_preserved",
            Self::TierManifestSkewRejected => "tier_manifest_skew_rejected",
            Self::Restore => "restore",
            Self::RestoreScopedBackupAdmitted => "restore_scoped_backup_admitted",
            Self::RestoreOutOfScopeRejected => "restore_out_of_scope_rejected",
            Self::RestorePublicationConflictRejected => "restore_publication_conflict_rejected",
            Self::RestoreMissingEdgeRejected => "restore_missing_edge_rejected",
            Self::RollingUpgrade => "rolling_upgrade",
            Self::RollingUpgradeTwoCapabilityAdmitted => "rolling_upgrade_two_capability_admitted",
            Self::RollingUpgradeMultiWriterRejected => "rolling_upgrade_multi_writer_rejected",
            Self::RollingUpgradeAdapterRejected => "rolling_upgrade_adapter_rejected",
            Self::AdapterParity => "adapter_parity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12VersionSkewReport {
    pub mixed_version_store_lane_count: u64,
    pub mixed_version_replica_lane_count: u64,
    pub rolling_upgrade_skew_rejection_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12AdmissionReport {
    pub accepted_count: u64,
    pub rejected_count: u64,
    pub relation_recheck_count: u64,
    pub edge_missing_rejection_count: u64,
    pub receipt_reuse_count: u64,
    pub manifest_index_rebuild_count: u64,
    pub manifest_entries_visited: u64,
    pub manifest_index_lookup_count: u64,
    pub manifest_digest_check_count: u64,
    pub manifest_publication_count: u64,
    pub manifest_recovery_record_count: u64,
    pub manifest_publication_gap_count: u64,
    pub manifest_digest_mismatch_count: u64,
    pub manifest_window_mismatch_count: u64,
    pub receipt_basis_mismatch_count: u64,
    pub artifact_row_scan_count: u64,
    pub malformed_frame_count: u64,
    pub adapter_hot_path_rejection_count: u64,
    pub adapter_maintenance_required_rejection_count: u64,
    pub adapter_out_of_scope_rejection_count: u64,
    pub admitted_native_count: u64,
    pub admitted_forward_backward_count: u64,
    pub admitted_adapter_count: u64,
    pub authoritative_partial_truth_rejection_count: u64,
    pub derived_reuse_incompatibility_count: u64,
    pub derived_rebuild_incompatibility_count: u64,
    pub derived_rebuild_required_count: u64,
    pub derived_invalidation_count: u64,
    pub derived_stale_version_rejection_count: u64,
    pub derived_rebuild_debt_count: u64,
    pub maintenance_compatibility_rebuild_admission_count: u64,
    pub maintenance_compatibility_rebuild_rejection_count: u64,
    pub derived_lane_plan_count: u64,
    pub derived_lane_reuse_count: u64,
    pub derived_lane_invalidation_count: u64,
    pub derived_lane_rejection_count: u64,
    pub derived_snapshot_reuse_count: u64,
    pub derived_delta_reuse_count: u64,
    pub derived_layout_basis_rejection_count: u64,
    pub derived_bulk_resume_rejection_count: u64,
    pub derived_maintenance_summary_rebuild_count: u64,
    pub tier_non_authority_preserved_count: u64,
    pub tier_manifest_rejection_count: u64,
    pub maintenance_lane_mismatch_rejection_count: u64,
    pub rolling_window_admission_count: u64,
    pub rolling_window_rejection_count: u64,
    pub rolling_multi_writer_rejection_count: u64,
    pub mixed_version_skew_count: u64,
    pub restore_accept_count: u64,
    pub restore_rejection_count: u64,
    pub restore_out_of_scope_scan_count: u64,
    pub restore_publication_conflict_rejection_count: u64,
    pub disaster_recovery_truth_window_count: u64,
    pub disaster_recovery_derived_window_count: u64,
}

impl Milestone12AdmissionReport {
    pub fn from_admission_counters(counters: &CompatibilityAdmissionCounters) -> Self {
        Self {
            accepted_count: counters.accepted_count(),
            rejected_count: counters.rejected_count(),
            relation_recheck_count: counters.relation_recheck_count(),
            edge_missing_rejection_count: counters.edge_missing_rejection_count(),
            receipt_reuse_count: counters.receipt_reuse_hit_count(),
            manifest_index_rebuild_count: counters.manifest_index_rebuild_count(),
            manifest_entries_visited: counters.manifest_entries_visited(),
            manifest_index_lookup_count: counters.manifest_index_lookup_count(),
            manifest_digest_check_count: counters.manifest_digest_check_count(),
            manifest_publication_count: counters.manifest_publication_count(),
            manifest_recovery_record_count: counters.manifest_recovery_record_count(),
            manifest_publication_gap_count: counters.manifest_publication_gap_count(),
            manifest_digest_mismatch_count: counters.manifest_digest_mismatch_count(),
            manifest_window_mismatch_count: counters.manifest_window_mismatch_count(),
            receipt_basis_mismatch_count: counters.receipt_basis_mismatch_count(),
            artifact_row_scan_count: counters.artifact_row_scan_count(),
            malformed_frame_count: counters.malformed_frame_count(),
            adapter_hot_path_rejection_count: counters.adapter_hot_path_rejection_count(),
            adapter_maintenance_required_rejection_count: counters
                .adapter_maintenance_required_rejection_count(),
            adapter_out_of_scope_rejection_count: counters.adapter_out_of_scope_rejection_count(),
            admitted_native_count: counters.admitted_native_count(),
            admitted_forward_backward_count: counters.admitted_forward_backward_count(),
            admitted_adapter_count: counters.admitted_adapter_count(),
            authoritative_partial_truth_rejection_count: counters
                .authoritative_partial_truth_rejection_count(),
            derived_reuse_incompatibility_count: counters.derived_reuse_incompatibility_count(),
            derived_rebuild_incompatibility_count: counters.derived_rebuild_incompatibility_count(),
            derived_rebuild_required_count: counters.derived_rebuild_required_count(),
            derived_invalidation_count: counters.derived_invalidation_count(),
            derived_stale_version_rejection_count: counters.derived_stale_version_rejection_count(),
            derived_rebuild_debt_count: counters.derived_rebuild_debt_count(),
            maintenance_compatibility_rebuild_admission_count: counters
                .maintenance_compatibility_rebuild_admission_count(),
            maintenance_compatibility_rebuild_rejection_count: counters
                .maintenance_compatibility_rebuild_rejection_count(),
            derived_lane_plan_count: counters.derived_lane_plan_count(),
            derived_lane_reuse_count: counters.derived_lane_reuse_count(),
            derived_lane_invalidation_count: counters.derived_lane_invalidation_count(),
            derived_lane_rejection_count: counters.derived_lane_rejection_count(),
            derived_snapshot_reuse_count: counters.derived_snapshot_reuse_count(),
            derived_delta_reuse_count: counters.derived_delta_reuse_count(),
            derived_layout_basis_rejection_count: counters.derived_layout_basis_rejection_count(),
            derived_bulk_resume_rejection_count: counters.derived_bulk_resume_rejection_count(),
            derived_maintenance_summary_rebuild_count: counters
                .derived_maintenance_summary_rebuild_count(),
            tier_non_authority_preserved_count: counters.tier_non_authority_preserved_count(),
            tier_manifest_rejection_count: counters.tier_manifest_rejection_count(),
            maintenance_lane_mismatch_rejection_count: counters
                .maintenance_lane_mismatch_rejection_count(),
            rolling_window_admission_count: counters.rolling_window_admission_count(),
            rolling_window_rejection_count: counters.rolling_window_rejection_count(),
            rolling_multi_writer_rejection_count: counters.rolling_multi_writer_rejection_count(),
            mixed_version_skew_count: counters.mixed_version_skew_count(),
            restore_accept_count: counters.restore_accept_count(),
            restore_rejection_count: counters.restore_rejection_count(),
            restore_out_of_scope_scan_count: counters.restore_out_of_scope_scan_count(),
            restore_publication_conflict_rejection_count: counters
                .restore_publication_conflict_rejection_count(),
            disaster_recovery_truth_window_count: counters.disaster_recovery_truth_window_count(),
            disaster_recovery_derived_window_count: counters
                .disaster_recovery_derived_window_count(),
        }
    }

    pub fn aggregate<'a>(
        reports: impl IntoIterator<Item = &'a Milestone12AdmissionReport>,
    ) -> Self {
        let mut aggregate = Self::zero();
        for report in reports {
            aggregate.accepted_count += report.accepted_count;
            aggregate.rejected_count += report.rejected_count;
            aggregate.relation_recheck_count += report.relation_recheck_count;
            aggregate.edge_missing_rejection_count += report.edge_missing_rejection_count;
            aggregate.receipt_reuse_count += report.receipt_reuse_count;
            aggregate.manifest_index_rebuild_count += report.manifest_index_rebuild_count;
            aggregate.manifest_entries_visited += report.manifest_entries_visited;
            aggregate.manifest_index_lookup_count += report.manifest_index_lookup_count;
            aggregate.manifest_digest_check_count += report.manifest_digest_check_count;
            aggregate.manifest_publication_count += report.manifest_publication_count;
            aggregate.manifest_recovery_record_count += report.manifest_recovery_record_count;
            aggregate.manifest_publication_gap_count += report.manifest_publication_gap_count;
            aggregate.manifest_digest_mismatch_count += report.manifest_digest_mismatch_count;
            aggregate.manifest_window_mismatch_count += report.manifest_window_mismatch_count;
            aggregate.receipt_basis_mismatch_count += report.receipt_basis_mismatch_count;
            aggregate.artifact_row_scan_count += report.artifact_row_scan_count;
            aggregate.malformed_frame_count += report.malformed_frame_count;
            aggregate.adapter_hot_path_rejection_count += report.adapter_hot_path_rejection_count;
            aggregate.adapter_maintenance_required_rejection_count +=
                report.adapter_maintenance_required_rejection_count;
            aggregate.adapter_out_of_scope_rejection_count +=
                report.adapter_out_of_scope_rejection_count;
            aggregate.admitted_native_count += report.admitted_native_count;
            aggregate.admitted_forward_backward_count += report.admitted_forward_backward_count;
            aggregate.admitted_adapter_count += report.admitted_adapter_count;
            aggregate.authoritative_partial_truth_rejection_count +=
                report.authoritative_partial_truth_rejection_count;
            aggregate.derived_reuse_incompatibility_count +=
                report.derived_reuse_incompatibility_count;
            aggregate.derived_rebuild_incompatibility_count +=
                report.derived_rebuild_incompatibility_count;
            aggregate.derived_rebuild_required_count += report.derived_rebuild_required_count;
            aggregate.derived_invalidation_count += report.derived_invalidation_count;
            aggregate.derived_stale_version_rejection_count +=
                report.derived_stale_version_rejection_count;
            aggregate.derived_rebuild_debt_count += report.derived_rebuild_debt_count;
            aggregate.maintenance_compatibility_rebuild_admission_count +=
                report.maintenance_compatibility_rebuild_admission_count;
            aggregate.maintenance_compatibility_rebuild_rejection_count +=
                report.maintenance_compatibility_rebuild_rejection_count;
            aggregate.derived_lane_plan_count += report.derived_lane_plan_count;
            aggregate.derived_lane_reuse_count += report.derived_lane_reuse_count;
            aggregate.derived_lane_invalidation_count += report.derived_lane_invalidation_count;
            aggregate.derived_lane_rejection_count += report.derived_lane_rejection_count;
            aggregate.derived_snapshot_reuse_count += report.derived_snapshot_reuse_count;
            aggregate.derived_delta_reuse_count += report.derived_delta_reuse_count;
            aggregate.derived_layout_basis_rejection_count +=
                report.derived_layout_basis_rejection_count;
            aggregate.derived_bulk_resume_rejection_count +=
                report.derived_bulk_resume_rejection_count;
            aggregate.derived_maintenance_summary_rebuild_count +=
                report.derived_maintenance_summary_rebuild_count;
            aggregate.tier_non_authority_preserved_count +=
                report.tier_non_authority_preserved_count;
            aggregate.tier_manifest_rejection_count += report.tier_manifest_rejection_count;
            aggregate.maintenance_lane_mismatch_rejection_count +=
                report.maintenance_lane_mismatch_rejection_count;
            aggregate.rolling_window_admission_count += report.rolling_window_admission_count;
            aggregate.rolling_window_rejection_count += report.rolling_window_rejection_count;
            aggregate.rolling_multi_writer_rejection_count +=
                report.rolling_multi_writer_rejection_count;
            aggregate.mixed_version_skew_count += report.mixed_version_skew_count;
            aggregate.restore_accept_count += report.restore_accept_count;
            aggregate.restore_rejection_count += report.restore_rejection_count;
            aggregate.restore_out_of_scope_scan_count += report.restore_out_of_scope_scan_count;
            aggregate.restore_publication_conflict_rejection_count +=
                report.restore_publication_conflict_rejection_count;
            aggregate.disaster_recovery_truth_window_count +=
                report.disaster_recovery_truth_window_count;
            aggregate.disaster_recovery_derived_window_count +=
                report.disaster_recovery_derived_window_count;
        }
        aggregate
    }

    pub fn has_counter_evidence(&self) -> bool {
        self.accepted_count != 0
            || self.rejected_count != 0
            || self.relation_recheck_count != 0
            || self.edge_missing_rejection_count != 0
            || self.receipt_reuse_count != 0
            || self.manifest_index_rebuild_count != 0
            || self.manifest_entries_visited != 0
            || self.manifest_index_lookup_count != 0
            || self.manifest_digest_check_count != 0
            || self.manifest_publication_count != 0
            || self.manifest_recovery_record_count != 0
            || self.manifest_publication_gap_count != 0
            || self.manifest_digest_mismatch_count != 0
            || self.manifest_window_mismatch_count != 0
            || self.receipt_basis_mismatch_count != 0
            || self.artifact_row_scan_count != 0
            || self.malformed_frame_count != 0
            || self.adapter_hot_path_rejection_count != 0
            || self.adapter_maintenance_required_rejection_count != 0
            || self.adapter_out_of_scope_rejection_count != 0
            || self.admitted_native_count != 0
            || self.admitted_forward_backward_count != 0
            || self.admitted_adapter_count != 0
            || self.authoritative_partial_truth_rejection_count != 0
            || self.derived_reuse_incompatibility_count != 0
            || self.derived_rebuild_incompatibility_count != 0
            || self.derived_rebuild_required_count != 0
            || self.derived_invalidation_count != 0
            || self.derived_stale_version_rejection_count != 0
            || self.derived_rebuild_debt_count != 0
            || self.maintenance_compatibility_rebuild_admission_count != 0
            || self.maintenance_compatibility_rebuild_rejection_count != 0
            || self.derived_lane_plan_count != 0
            || self.derived_lane_reuse_count != 0
            || self.derived_lane_invalidation_count != 0
            || self.derived_lane_rejection_count != 0
            || self.derived_snapshot_reuse_count != 0
            || self.derived_delta_reuse_count != 0
            || self.derived_layout_basis_rejection_count != 0
            || self.derived_bulk_resume_rejection_count != 0
            || self.derived_maintenance_summary_rebuild_count != 0
            || self.tier_non_authority_preserved_count != 0
            || self.tier_manifest_rejection_count != 0
            || self.maintenance_lane_mismatch_rejection_count != 0
            || self.rolling_window_admission_count != 0
            || self.rolling_window_rejection_count != 0
            || self.rolling_multi_writer_rejection_count != 0
            || self.mixed_version_skew_count != 0
            || self.restore_accept_count != 0
            || self.restore_rejection_count != 0
            || self.restore_out_of_scope_scan_count != 0
            || self.restore_publication_conflict_rejection_count != 0
            || self.disaster_recovery_truth_window_count != 0
            || self.disaster_recovery_derived_window_count != 0
    }

    fn zero() -> Self {
        Self {
            accepted_count: 0,
            rejected_count: 0,
            relation_recheck_count: 0,
            edge_missing_rejection_count: 0,
            receipt_reuse_count: 0,
            manifest_index_rebuild_count: 0,
            manifest_entries_visited: 0,
            manifest_index_lookup_count: 0,
            manifest_digest_check_count: 0,
            manifest_publication_count: 0,
            manifest_recovery_record_count: 0,
            manifest_publication_gap_count: 0,
            manifest_digest_mismatch_count: 0,
            manifest_window_mismatch_count: 0,
            receipt_basis_mismatch_count: 0,
            artifact_row_scan_count: 0,
            malformed_frame_count: 0,
            adapter_hot_path_rejection_count: 0,
            adapter_maintenance_required_rejection_count: 0,
            adapter_out_of_scope_rejection_count: 0,
            admitted_native_count: 0,
            admitted_forward_backward_count: 0,
            admitted_adapter_count: 0,
            authoritative_partial_truth_rejection_count: 0,
            derived_reuse_incompatibility_count: 0,
            derived_rebuild_incompatibility_count: 0,
            derived_rebuild_required_count: 0,
            derived_invalidation_count: 0,
            derived_stale_version_rejection_count: 0,
            derived_rebuild_debt_count: 0,
            maintenance_compatibility_rebuild_admission_count: 0,
            maintenance_compatibility_rebuild_rejection_count: 0,
            derived_lane_plan_count: 0,
            derived_lane_reuse_count: 0,
            derived_lane_invalidation_count: 0,
            derived_lane_rejection_count: 0,
            derived_snapshot_reuse_count: 0,
            derived_delta_reuse_count: 0,
            derived_layout_basis_rejection_count: 0,
            derived_bulk_resume_rejection_count: 0,
            derived_maintenance_summary_rebuild_count: 0,
            tier_non_authority_preserved_count: 0,
            tier_manifest_rejection_count: 0,
            maintenance_lane_mismatch_rejection_count: 0,
            rolling_window_admission_count: 0,
            rolling_window_rejection_count: 0,
            rolling_multi_writer_rejection_count: 0,
            mixed_version_skew_count: 0,
            restore_accept_count: 0,
            restore_rejection_count: 0,
            restore_out_of_scope_scan_count: 0,
            restore_publication_conflict_rejection_count: 0,
            disaster_recovery_truth_window_count: 0,
            disaster_recovery_derived_window_count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CounterContract {
    pub counter_names: Vec<&'static str>,
}

impl Milestone12CounterContract {
    pub fn phase_1() -> Self {
        Self {
            counter_names: MILESTONE_12_COUNTER_NAMES.to_vec(),
        }
    }

    pub fn validate_report(
        &self,
        _report: &Milestone12AdmissionReport,
    ) -> Result<(), Milestone12CounterContractViolation> {
        for counter in MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES {
            if !self.counter_names.contains(counter) {
                return Err(Milestone12CounterContractViolation::MissingReportCounter);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CounterContractViolation {
    MissingReportCounter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ComplexityPathStatus {
    pub verified: bool,
    pub detail: String,
}

impl Milestone12ComplexityPathStatus {
    pub fn verified(detail: impl Into<String>) -> Self {
        Self {
            verified: true,
            detail: detail.into(),
        }
    }

    pub fn debt(detail: impl Into<String>) -> Self {
        Self {
            verified: false,
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ComplexitySurface {
    pub relation_recheck: Milestone12ComplexityPathStatus,
    pub index_lookup: Milestone12ComplexityPathStatus,
    pub adapter_cost: Milestone12ComplexityPathStatus,
    pub restore_scan: Milestone12ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationSummary {
    pub phase_1_registry_declared: bool,
    pub phase_1_counters_declared: bool,
    pub phase_1_witness_boundaries_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationBundle {
    pub summary: Milestone12CertificationSummary,
    pub counter_contract: Milestone12CounterContract,
    pub complexity_surface: Milestone12ComplexitySurface,
    pub admission_report: Milestone12AdmissionReport,
    pub matrix_rows: Vec<Milestone12CompatibilityMatrixRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ArtifactFormatEvolutionEvidence {
    lane_count: u64,
}

impl Milestone12ArtifactFormatEvolutionEvidence {
    pub fn lane_count(&self) -> u64 {
        self.lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12RollingCompatibilityEvidence {
    admitted_lane_count: u64,
    rejected_lane_count: u64,
}

impl Milestone12RollingCompatibilityEvidence {
    pub fn admitted_lane_count(&self) -> u64 {
        self.admitted_lane_count
    }

    pub fn rejected_lane_count(&self) -> u64 {
        self.rejected_lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12RestoreCompatibilityEvidence {
    admitted_lane_count: u64,
    rejected_lane_count: u64,
}

impl Milestone12RestoreCompatibilityEvidence {
    pub fn admitted_lane_count(&self) -> u64 {
        self.admitted_lane_count
    }

    pub fn rejected_lane_count(&self) -> u64 {
        self.rejected_lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12DerivedCompatibilityEvidence {
    admitted_lane_count: u64,
    non_admitted_lane_count: u64,
}

impl Milestone12DerivedCompatibilityEvidence {
    pub fn admitted_lane_count(&self) -> u64 {
        self.admitted_lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationEvidenceBundle {
    admission_report: Milestone12AdmissionReport,
    compatibility_matrix: Milestone12CompatibilityMatrix,
    version_skew_report: Milestone12VersionSkewReport,
    complexity_surface: Milestone12ComplexitySurface,
    lane_outcomes: Vec<Milestone12CertificationLaneOutcome>,
    run_summary: Milestone12CertificationRunSummary,
    artifact_format_evidence: Milestone12ArtifactFormatEvolutionEvidence,
    rolling_evidence: Milestone12RollingCompatibilityEvidence,
    restore_evidence: Milestone12RestoreCompatibilityEvidence,
    derived_evidence: Milestone12DerivedCompatibilityEvidence,
}

impl Milestone12CertificationEvidenceBundle {
    pub fn from_parts(
        admission_report: Milestone12AdmissionReport,
        compatibility_matrix: Milestone12CompatibilityMatrix,
        version_skew_report: Milestone12VersionSkewReport,
        complexity_surface: Milestone12ComplexitySurface,
        mut lane_outcomes: Vec<Milestone12CertificationLaneOutcome>,
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        let matrix_ids = compatibility_matrix
            .entries()
            .iter()
            .map(|entry| entry.lane_id().clone())
            .collect::<BTreeSet<_>>();
        let outcome_ids = lane_outcomes
            .iter()
            .map(|outcome| outcome.lane_id().clone())
            .collect::<BTreeSet<_>>();
        if matrix_ids != outcome_ids {
            return Err(Milestone12CertificationLaneRejection::MatrixLaneMismatch);
        }
        for outcome in &lane_outcomes {
            if !outcome.counters().has_counter_evidence() {
                return Err(Milestone12CertificationLaneRejection::CounterEvidenceMissing);
            }
        }
        lane_outcomes.sort_by_key(|outcome| outcome.lane_id().clone());
        let run_summary = Milestone12CertificationRunSummary::from_outcomes(&lane_outcomes);
        let artifact_format_evidence = Milestone12ArtifactFormatEvolutionEvidence {
            lane_count: lane_outcomes.len() as u64,
        };
        let rolling_evidence = rolling_evidence(&lane_outcomes);
        let restore_evidence = restore_evidence(&lane_outcomes);
        let derived_evidence = derived_evidence(&lane_outcomes);
        Ok(Self {
            admission_report,
            compatibility_matrix,
            version_skew_report,
            complexity_surface,
            lane_outcomes,
            run_summary,
            artifact_format_evidence,
            rolling_evidence,
            restore_evidence,
            derived_evidence,
        })
    }

    pub fn lane_outcomes(&self) -> &[Milestone12CertificationLaneOutcome] {
        &self.lane_outcomes
    }

    pub fn run_summary(&self) -> &Milestone12CertificationRunSummary {
        &self.run_summary
    }

    pub fn rolling_evidence(&self) -> &Milestone12RollingCompatibilityEvidence {
        &self.rolling_evidence
    }

    pub fn restore_evidence(&self) -> &Milestone12RestoreCompatibilityEvidence {
        &self.restore_evidence
    }
}

fn rolling_evidence(
    outcomes: &[Milestone12CertificationLaneOutcome],
) -> Milestone12RollingCompatibilityEvidence {
    let mut admitted_lane_count = 0;
    let mut rejected_lane_count = 0;
    for outcome in outcomes {
        match outcome.lane_kind() {
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted => {
                admitted_lane_count += 1;
            }
            Milestone12CertificationLaneKind::RollingMultiWriterRejected
            | Milestone12CertificationLaneKind::RollingMissingEdgeRejected
            | Milestone12CertificationLaneKind::RollingAdapterEdgeRejected => {
                rejected_lane_count += 1;
            }
            _ => {}
        }
    }
    Milestone12RollingCompatibilityEvidence {
        admitted_lane_count,
        rejected_lane_count,
    }
}

fn restore_evidence(
    outcomes: &[Milestone12CertificationLaneOutcome],
) -> Milestone12RestoreCompatibilityEvidence {
    let mut admitted_lane_count = 0;
    let mut rejected_lane_count = 0;
    for outcome in outcomes {
        match outcome.lane_kind() {
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted => {
                admitted_lane_count += 1;
            }
            Milestone12CertificationLaneKind::RestoreOutOfScopeRejected
            | Milestone12CertificationLaneKind::RestorePublicationConflictRejected
            | Milestone12CertificationLaneKind::RestoreMissingEdgeRejected => {
                rejected_lane_count += 1;
            }
            _ => {}
        }
    }
    Milestone12RestoreCompatibilityEvidence {
        admitted_lane_count,
        rejected_lane_count,
    }
}

fn derived_evidence(
    outcomes: &[Milestone12CertificationLaneOutcome],
) -> Milestone12DerivedCompatibilityEvidence {
    let mut admitted_lane_count = 0;
    let mut non_admitted_lane_count = 0;
    for outcome in outcomes {
        match outcome.lane_kind() {
            Milestone12CertificationLaneKind::DerivedSnapshotReuseAccepted
            | Milestone12CertificationLaneKind::TierManifestNonAuthorityPreserved => {
                admitted_lane_count += 1;
            }
            Milestone12CertificationLaneKind::DerivedLayoutBasisRejected
            | Milestone12CertificationLaneKind::DerivedBulkResumeRejected => {
                non_admitted_lane_count += 1;
            }
            _ => {}
        }
    }
    Milestone12DerivedCompatibilityEvidence {
        admitted_lane_count,
        non_admitted_lane_count,
    }
}
