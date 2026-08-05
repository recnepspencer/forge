use super::*;

impl CompatibilityAdmissionCounters {
    pub fn accepted_count(&self) -> u64 {
        self.accepted_count
    }

    pub fn rejected_count(&self) -> u64 {
        self.rejected_count
    }

    pub fn manifest_index_rebuild_count(&self) -> u64 {
        self.manifest_index_rebuild_count
    }

    pub fn manifest_entries_visited(&self) -> u64 {
        self.manifest_entries_visited
    }

    pub fn manifest_index_lookup_count(&self) -> u64 {
        self.manifest_index_lookup_count
    }

    pub fn manifest_digest_check_count(&self) -> u64 {
        self.manifest_digest_check_count
    }

    pub fn manifest_publication_count(&self) -> u64 {
        self.manifest_publication_count
    }

    pub fn manifest_recovery_record_count(&self) -> u64 {
        self.manifest_recovery_record_count
    }

    pub fn manifest_publication_gap_count(&self) -> u64 {
        self.manifest_publication_gap_count
    }

    pub fn manifest_digest_mismatch_count(&self) -> u64 {
        self.manifest_digest_mismatch_count
    }

    pub fn manifest_window_mismatch_count(&self) -> u64 {
        self.manifest_window_mismatch_count
    }

    pub fn relation_recheck_count(&self) -> u64 {
        self.relation_recheck_count
    }

    pub fn edge_missing_rejection_count(&self) -> u64 {
        self.edge_missing_rejection_count
    }

    pub fn receipt_reuse_hit_count(&self) -> u64 {
        self.receipt_reuse_hit_count
    }

    pub fn receipt_reuse_rejection_count(&self) -> u64 {
        self.receipt_reuse_rejection_count
    }

    pub fn receipt_basis_mismatch_count(&self) -> u64 {
        self.receipt_basis_mismatch_count
    }

    pub fn artifact_row_scan_count(&self) -> u64 {
        self.artifact_row_scan_count
    }

    pub fn malformed_frame_count(&self) -> u64 {
        self.malformed_frame_count
    }

    pub fn adapter_cost_class_count(&self) -> u64 {
        self.adapter_cost_class_count
    }

    pub fn adapter_parity_failure_count(&self) -> u64 {
        self.adapter_parity_failure_count
    }

    pub fn adapter_inline_count(&self) -> u64 {
        self.adapter_inline_count
    }

    pub fn adapter_batch_count(&self) -> u64 {
        self.adapter_batch_count
    }

    pub fn adapter_maintenance_scheduled_count(&self) -> u64 {
        self.adapter_maintenance_scheduled_count
    }

    pub fn adapter_input_record_count(&self) -> u64 {
        self.adapter_input_record_count
    }

    pub fn adapter_output_record_count(&self) -> u64 {
        self.adapter_output_record_count
    }

    pub fn adapter_allocation_scope_count(&self) -> u64 {
        self.adapter_allocation_scope_count
    }

    pub fn adapter_hot_path_rejection_count(&self) -> u64 {
        self.adapter_hot_path_rejection_count
    }

    pub fn adapter_maintenance_required_rejection_count(&self) -> u64 {
        self.adapter_maintenance_required_rejection_count
    }

    pub fn adapter_out_of_scope_rejection_count(&self) -> u64 {
        self.adapter_out_of_scope_rejection_count
    }

    pub fn admitted_native_count(&self) -> u64 {
        self.admitted_native_count
    }

    pub fn admitted_forward_backward_count(&self) -> u64 {
        self.admitted_forward_backward_count
    }

    pub fn admitted_adapter_count(&self) -> u64 {
        self.admitted_adapter_count
    }

    pub fn authoritative_partial_truth_rejection_count(&self) -> u64 {
        self.authoritative_partial_truth_rejection_count
    }

    pub fn derived_reuse_incompatibility_count(&self) -> u64 {
        self.derived_reuse_incompatibility_count
    }

    pub fn derived_rebuild_required_count(&self) -> u64 {
        self.derived_rebuild_required_count
    }

    pub fn derived_rebuild_incompatibility_count(&self) -> u64 {
        self.derived_rebuild_incompatibility_count
    }

    pub fn derived_invalidation_count(&self) -> u64 {
        self.derived_invalidation_count
    }

    pub fn derived_stale_version_rejection_count(&self) -> u64 {
        self.derived_stale_version_rejection_count
    }

    pub fn derived_rebuild_debt_count(&self) -> u64 {
        self.derived_rebuild_debt_count
    }

    pub fn maintenance_compatibility_rebuild_admission_count(&self) -> u64 {
        self.maintenance_compatibility_rebuild_admission_count
    }

    pub fn maintenance_compatibility_rebuild_rejection_count(&self) -> u64 {
        self.maintenance_compatibility_rebuild_rejection_count
    }

    pub fn derived_lane_plan_count(&self) -> u64 {
        self.derived_lane_plan_count
    }

    pub fn derived_lane_reuse_count(&self) -> u64 {
        self.derived_lane_reuse_count
    }

    pub fn derived_lane_invalidation_count(&self) -> u64 {
        self.derived_lane_invalidation_count
    }

    pub fn derived_lane_rejection_count(&self) -> u64 {
        self.derived_lane_rejection_count
    }

    pub fn derived_snapshot_reuse_count(&self) -> u64 {
        self.derived_snapshot_reuse_count
    }

    pub fn derived_delta_reuse_count(&self) -> u64 {
        self.derived_delta_reuse_count
    }

    pub fn derived_layout_basis_rejection_count(&self) -> u64 {
        self.derived_layout_basis_rejection_count
    }

    pub fn derived_bulk_resume_rejection_count(&self) -> u64 {
        self.derived_bulk_resume_rejection_count
    }

    pub fn derived_maintenance_summary_rebuild_count(&self) -> u64 {
        self.derived_maintenance_summary_rebuild_count
    }

    pub fn tier_non_authority_preserved_count(&self) -> u64 {
        self.tier_non_authority_preserved_count
    }

    pub fn tier_manifest_rejection_count(&self) -> u64 {
        self.tier_manifest_rejection_count
    }

    pub fn maintenance_lane_mismatch_rejection_count(&self) -> u64 {
        self.maintenance_lane_mismatch_rejection_count
    }

    pub fn rolling_window_admission_count(&self) -> u64 {
        self.rolling_window_admission_count
    }

    pub fn rolling_window_rejection_count(&self) -> u64 {
        self.rolling_window_rejection_count
    }

    pub fn rolling_multi_writer_rejection_count(&self) -> u64 {
        self.rolling_multi_writer_rejection_count
    }

    pub fn mixed_version_skew_count(&self) -> u64 {
        self.mixed_version_skew_count
    }

    pub fn restore_accept_count(&self) -> u64 {
        self.restore_accept_count
    }

    pub fn restore_rejection_count(&self) -> u64 {
        self.restore_rejection_count
    }

    pub fn restore_out_of_scope_scan_count(&self) -> u64 {
        self.restore_out_of_scope_scan_count
    }

    pub fn restore_publication_conflict_rejection_count(&self) -> u64 {
        self.restore_publication_conflict_rejection_count
    }

    pub fn disaster_recovery_truth_window_count(&self) -> u64 {
        self.disaster_recovery_truth_window_count
    }

    pub fn disaster_recovery_derived_window_count(&self) -> u64 {
        self.disaster_recovery_derived_window_count
    }

}
