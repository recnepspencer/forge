use serde::Serialize;

use crate::compatibility::CompatibilityAdmissionCounters;

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
    pub adapter_cost_class_count: u64,
    pub adapter_inline_count: u64,
    pub adapter_batch_count: u64,
    pub adapter_maintenance_scheduled_count: u64,
    pub adapter_parity_failure_count: u64,
    pub adapter_hot_path_rejection_count: u64,
    pub adapter_maintenance_required_rejection_count: u64,
    pub adapter_out_of_scope_rejection_count: u64,
    pub adapter_input_record_count: u64,
    pub adapter_output_record_count: u64,
    pub adapter_allocation_scope_count: u64,
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
            adapter_cost_class_count: counters.adapter_cost_class_count(),
            adapter_inline_count: counters.adapter_inline_count(),
            adapter_batch_count: counters.adapter_batch_count(),
            adapter_maintenance_scheduled_count: counters.adapter_maintenance_scheduled_count(),
            adapter_parity_failure_count: counters.adapter_parity_failure_count(),
            adapter_hot_path_rejection_count: counters.adapter_hot_path_rejection_count(),
            adapter_maintenance_required_rejection_count: counters
                .adapter_maintenance_required_rejection_count(),
            adapter_out_of_scope_rejection_count: counters.adapter_out_of_scope_rejection_count(),
            adapter_input_record_count: counters.adapter_input_record_count(),
            adapter_output_record_count: counters.adapter_output_record_count(),
            adapter_allocation_scope_count: counters.adapter_allocation_scope_count(),
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
            aggregate.adapter_cost_class_count += report.adapter_cost_class_count;
            aggregate.adapter_inline_count += report.adapter_inline_count;
            aggregate.adapter_batch_count += report.adapter_batch_count;
            aggregate.adapter_maintenance_scheduled_count +=
                report.adapter_maintenance_scheduled_count;
            aggregate.adapter_parity_failure_count += report.adapter_parity_failure_count;
            aggregate.adapter_hot_path_rejection_count += report.adapter_hot_path_rejection_count;
            aggregate.adapter_maintenance_required_rejection_count +=
                report.adapter_maintenance_required_rejection_count;
            aggregate.adapter_out_of_scope_rejection_count +=
                report.adapter_out_of_scope_rejection_count;
            aggregate.adapter_input_record_count += report.adapter_input_record_count;
            aggregate.adapter_output_record_count += report.adapter_output_record_count;
            aggregate.adapter_allocation_scope_count += report.adapter_allocation_scope_count;
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
            || self.adapter_cost_class_count != 0
            || self.adapter_inline_count != 0
            || self.adapter_batch_count != 0
            || self.adapter_maintenance_scheduled_count != 0
            || self.adapter_parity_failure_count != 0
            || self.adapter_hot_path_rejection_count != 0
            || self.adapter_maintenance_required_rejection_count != 0
            || self.adapter_out_of_scope_rejection_count != 0
            || self.adapter_input_record_count != 0
            || self.adapter_output_record_count != 0
            || self.adapter_allocation_scope_count != 0
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
            adapter_cost_class_count: 0,
            adapter_inline_count: 0,
            adapter_batch_count: 0,
            adapter_maintenance_scheduled_count: 0,
            adapter_parity_failure_count: 0,
            adapter_hot_path_rejection_count: 0,
            adapter_maintenance_required_rejection_count: 0,
            adapter_out_of_scope_rejection_count: 0,
            adapter_input_record_count: 0,
            adapter_output_record_count: 0,
            adapter_allocation_scope_count: 0,
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
