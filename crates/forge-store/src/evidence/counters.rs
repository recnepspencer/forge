use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct CanonicalizationMetrics {
    pub canonicalization_item_count: u64,
    pub canonicalization_duplicate_collapse_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct StoreCounterSnapshot {
    pub durable_mode_selection_count: u64,
    pub embedded_mode_selection_count: u64,
    pub absent_mode_selection_count: u64,
    pub hosted_runtime_start_count: u64,
    pub hosted_runtime_stop_count: u64,
    pub external_commit_intake_count: u64,
    pub external_checkpoint_intake_count: u64,
    pub embedded_checkpoint_authority_rejection_count: u64,
    pub cross_mode_canonical_boundary_reuse_count: u64,
    pub mode_misuse_rejection_count: u64,
    pub absent_mode_store_touch_count: u64,
    pub authoritative_commit_append_count: u64,
    pub authoritative_commit_fetch_count: u64,
    pub commit_parent_record_write_count: u64,
    pub branch_head_write_count: u64,
    pub authoritative_digest_write_count: u64,
    pub commit_support_publication_count: u64,
    pub commit_support_publication_gap_count: u64,
    pub commit_support_summary_build_count: u64,
    pub schema_boundary_fetch_count: u64,
    pub schema_boundary_index_lookup_count: u64,
    pub schema_boundary_rows_read: u64,
    pub schema_boundary_resolution_count: u64,
    pub lineage_lookup_count: u64,
    pub lineage_identity_lookup_count: u64,
    pub lineage_event_rows_read: u64,
    pub lineage_resolution_breadth: u64,
    pub cursor_resume_count: u64,
    pub cursor_identity_lookup_count: u64,
    pub cursor_resume_support_rows_read: u64,
    pub cursor_resume_step_count: u64,
    pub cursor_ack_count: u64,
    pub cursor_equivalence_reject_count: u64,
    pub cursor_regression_reject_count: u64,
    pub subscriber_checkpoint_write_count: u64,
    pub embedded_checkpoint_fetch_count: u64,
    pub embedded_checkpoint_index_lookup_count: u64,
    pub embedded_checkpoint_basis_read_count: u64,
    pub checkpoint_shape_reject_count: u64,
    pub support_artifact_recovery_gap_count: u64,
    pub state_delta_apply_count: u64,
    pub state_delta_touched_family_count: u64,
    pub state_delta_touched_record_count: u64,
    pub state_clone_fallback_count: u64,
    pub canonicalization_item_count: u64,
    pub canonicalization_duplicate_collapse_count: u64,
    pub authoritative_fetch_verification_count: u64,
    pub authoritative_fetch_verification_failure_count: u64,
    pub wal_record_append_count: u64,
    pub wal_record_scan_count: u64,
    pub wal_record_decode_failure_count: u64,
    pub durable_mutation_admit_count: u64,
    pub durable_commit_acknowledged_count: u64,
    pub durable_commit_recovered_count: u64,
    pub durable_commit_duplicate_suppression_count: u64,
    pub durable_commit_unacknowledged_discard_count: u64,
    pub recovery_requires_full_rebuild_count: u64,
    pub recovery_failure_count: u64,
    pub durable_frame_scan_count: u64,
    pub durable_frame_reject_count: u64,
    pub durable_truncated_tail_count: u64,
    pub durable_torn_write_count: u64,
    pub durable_barrier_verified_count: u64,
    pub durable_ack_barrier_violation_count: u64,
    pub recovery_source_precedence_resolution_count: u64,
    pub recovery_source_precedence_fallback_count: u64,
    pub recovery_quiescent_restart_count: u64,
    pub recovery_non_quiescent_restart_count: u64,
    pub recovery_quarantine_count: u64,
    pub recovery_salvage_count: u64,
    pub interrupted_maintenance_recovery_count: u64,
    pub backup_restore_compatibility_reject_count: u64,
    pub snapshot_capture_count: u64,
    pub snapshot_capture_record_count: u64,
    pub snapshot_capture_byte_count: u64,
    pub snapshot_read_count: u64,
    pub snapshot_read_record_count: u64,
    pub snapshot_read_tail_commit_count: u64,
    pub snapshot_read_tail_replay_count: u64,
    pub snapshot_restore_count: u64,
    pub snapshot_restore_tail_commit_count: u64,
    pub snapshot_restore_tail_replay_count: u64,
    pub snapshot_rebuild_count: u64,
    pub snapshot_rebuild_record_count: u64,
    pub snapshot_integrity_failure_count: u64,
    pub snapshot_basis_mismatch_count: u64,
    pub branch_create_count: u64,
    pub branch_base_reuse_count: u64,
    pub branch_base_copy_count: u64,
    pub branch_hidden_full_base_materialization_count: u64,
    pub branch_delta_read_count: u64,
    pub branch_delta_layers_traversed_count: u64,
    pub branch_delta_read_record_count: u64,
    pub branch_delta_replay_commit_count: u64,
    pub branch_delta_authority_replay_fallback_count: u64,
    pub branch_delta_rewrite_count: u64,
    pub branch_delta_rewrite_layers_replaced_count: u64,
    pub branch_delta_rewrite_record_count: u64,
    pub branch_delta_hidden_full_stack_rewrite_count: u64,
    pub branch_delta_merge_path_search_count: u64,
    pub branch_delta_rebuild_count: u64,
    pub branch_delta_rebuild_record_count: u64,
    pub branch_delta_integrity_failure_count: u64,
    pub concurrent_artifact_boundary_rejection_count: u64,
    pub aspect_layout_plan_count: u64,
    pub aspect_layout_admitted_count: u64,
    pub aspect_layout_fallback_count: u64,
    pub aspect_layout_rejected_count: u64,
    pub aspect_layout_slice_read_count: u64,
    pub aspect_layout_block_decode_count: u64,
    pub aspect_layout_control_replay_breadth: u64,
    pub aspect_layout_whole_state_fallback_count: u64,
    pub structural_block_lookup_count: u64,
    pub structural_block_reuse_admission_count: u64,
    pub structural_block_reuse_hit_count: u64,
    pub structural_block_reuse_miss_count: u64,
    pub chunk_model_freeze_count: u64,
    pub physical_chunk_export_count: u64,
    pub physical_chunk_width_count: u64,
    pub physical_chunk_determinism_violation_count: u64,
    pub milestone_6_proof_only_prepare_count: u64,
    pub milestone_6_on_demand_materialize_count: u64,
    pub milestone_6_policy_eager_resolution_count: u64,
    pub milestone_6_policy_eager_publish_count: u64,
    pub milestone_6_policy_eager_reuse_existing_count: u64,
    pub milestone_7_layout_reference_admission_count: u64,
    pub milestone_9_physical_chunk_reference_admission_count: u64,
    pub bulk_program_plan_count: u64,
    pub bulk_source_manifest_member_count: u64,
    pub bulk_source_manifest_stream_pass_count: u64,
    pub bulk_transform_partition_count: u64,
    pub bulk_chunk_plan_count: u64,
    pub bulk_chunk_execute_count: u64,
    pub bulk_checkpoint_write_count: u64,
    pub bulk_chunk_witness_write_count: u64,
    pub bulk_resume_index_lookup_count: u64,
    pub bulk_chunk_resume_count: u64,
    pub bulk_chunk_commit_count: u64,
    pub bulk_chunk_width_units: u64,
    pub bulk_peak_in_flight_memory_units: u64,
    pub bulk_fallback_path_count: u64,
    pub bulk_fallback_breadth_units: u64,
}

#[derive(Debug, Default)]
pub(crate) struct StoreCounters {
    durable_mode_selection_count: AtomicU64,
    embedded_mode_selection_count: AtomicU64,
    absent_mode_selection_count: AtomicU64,
    hosted_runtime_start_count: AtomicU64,
    hosted_runtime_stop_count: AtomicU64,
    external_commit_intake_count: AtomicU64,
    external_checkpoint_intake_count: AtomicU64,
    embedded_checkpoint_authority_rejection_count: AtomicU64,
    cross_mode_canonical_boundary_reuse_count: AtomicU64,
    mode_misuse_rejection_count: AtomicU64,
    absent_mode_store_touch_count: AtomicU64,
    authoritative_commit_append_count: AtomicU64,
    authoritative_commit_fetch_count: AtomicU64,
    commit_parent_record_write_count: AtomicU64,
    branch_head_write_count: AtomicU64,
    authoritative_digest_write_count: AtomicU64,
    commit_support_publication_count: AtomicU64,
    commit_support_publication_gap_count: AtomicU64,
    commit_support_summary_build_count: AtomicU64,
    schema_boundary_fetch_count: AtomicU64,
    schema_boundary_index_lookup_count: AtomicU64,
    schema_boundary_rows_read: AtomicU64,
    schema_boundary_resolution_count: AtomicU64,
    lineage_lookup_count: AtomicU64,
    lineage_identity_lookup_count: AtomicU64,
    lineage_event_rows_read: AtomicU64,
    lineage_resolution_breadth: AtomicU64,
    cursor_resume_count: AtomicU64,
    cursor_identity_lookup_count: AtomicU64,
    cursor_resume_support_rows_read: AtomicU64,
    cursor_resume_step_count: AtomicU64,
    cursor_ack_count: AtomicU64,
    cursor_equivalence_reject_count: AtomicU64,
    cursor_regression_reject_count: AtomicU64,
    subscriber_checkpoint_write_count: AtomicU64,
    embedded_checkpoint_fetch_count: AtomicU64,
    embedded_checkpoint_index_lookup_count: AtomicU64,
    embedded_checkpoint_basis_read_count: AtomicU64,
    checkpoint_shape_reject_count: AtomicU64,
    support_artifact_recovery_gap_count: AtomicU64,
    state_delta_apply_count: AtomicU64,
    state_delta_touched_family_count: AtomicU64,
    state_delta_touched_record_count: AtomicU64,
    state_clone_fallback_count: AtomicU64,
    canonicalization_item_count: AtomicU64,
    canonicalization_duplicate_collapse_count: AtomicU64,
    authoritative_fetch_verification_count: AtomicU64,
    authoritative_fetch_verification_failure_count: AtomicU64,
    wal_record_append_count: AtomicU64,
    wal_record_scan_count: AtomicU64,
    wal_record_decode_failure_count: AtomicU64,
    durable_mutation_admit_count: AtomicU64,
    durable_commit_acknowledged_count: AtomicU64,
    durable_commit_recovered_count: AtomicU64,
    durable_commit_duplicate_suppression_count: AtomicU64,
    durable_commit_unacknowledged_discard_count: AtomicU64,
    recovery_requires_full_rebuild_count: AtomicU64,
    recovery_failure_count: AtomicU64,
    durable_frame_scan_count: AtomicU64,
    durable_frame_reject_count: AtomicU64,
    durable_truncated_tail_count: AtomicU64,
    durable_torn_write_count: AtomicU64,
    durable_barrier_verified_count: AtomicU64,
    durable_ack_barrier_violation_count: AtomicU64,
    recovery_source_precedence_resolution_count: AtomicU64,
    recovery_source_precedence_fallback_count: AtomicU64,
    recovery_quiescent_restart_count: AtomicU64,
    recovery_non_quiescent_restart_count: AtomicU64,
    recovery_quarantine_count: AtomicU64,
    recovery_salvage_count: AtomicU64,
    interrupted_maintenance_recovery_count: AtomicU64,
    backup_restore_compatibility_reject_count: AtomicU64,
    snapshot_capture_count: AtomicU64,
    snapshot_capture_record_count: AtomicU64,
    snapshot_capture_byte_count: AtomicU64,
    snapshot_read_count: AtomicU64,
    snapshot_read_record_count: AtomicU64,
    snapshot_read_tail_commit_count: AtomicU64,
    snapshot_read_tail_replay_count: AtomicU64,
    snapshot_restore_count: AtomicU64,
    snapshot_restore_tail_commit_count: AtomicU64,
    snapshot_restore_tail_replay_count: AtomicU64,
    snapshot_rebuild_count: AtomicU64,
    snapshot_rebuild_record_count: AtomicU64,
    snapshot_integrity_failure_count: AtomicU64,
    snapshot_basis_mismatch_count: AtomicU64,
    branch_create_count: AtomicU64,
    branch_base_reuse_count: AtomicU64,
    branch_base_copy_count: AtomicU64,
    branch_hidden_full_base_materialization_count: AtomicU64,
    branch_delta_read_count: AtomicU64,
    branch_delta_layers_traversed_count: AtomicU64,
    branch_delta_read_record_count: AtomicU64,
    branch_delta_replay_commit_count: AtomicU64,
    branch_delta_authority_replay_fallback_count: AtomicU64,
    branch_delta_rewrite_count: AtomicU64,
    branch_delta_rewrite_layers_replaced_count: AtomicU64,
    branch_delta_rewrite_record_count: AtomicU64,
    branch_delta_hidden_full_stack_rewrite_count: AtomicU64,
    branch_delta_merge_path_search_count: AtomicU64,
    branch_delta_rebuild_count: AtomicU64,
    branch_delta_rebuild_record_count: AtomicU64,
    branch_delta_integrity_failure_count: AtomicU64,
    concurrent_artifact_boundary_rejection_count: AtomicU64,
    aspect_layout_plan_count: AtomicU64,
    aspect_layout_admitted_count: AtomicU64,
    aspect_layout_fallback_count: AtomicU64,
    aspect_layout_rejected_count: AtomicU64,
    aspect_layout_slice_read_count: AtomicU64,
    aspect_layout_block_decode_count: AtomicU64,
    aspect_layout_control_replay_breadth: AtomicU64,
    aspect_layout_whole_state_fallback_count: AtomicU64,
    structural_block_lookup_count: AtomicU64,
    structural_block_reuse_admission_count: AtomicU64,
    structural_block_reuse_hit_count: AtomicU64,
    structural_block_reuse_miss_count: AtomicU64,
    chunk_model_freeze_count: AtomicU64,
    physical_chunk_export_count: AtomicU64,
    physical_chunk_width_count: AtomicU64,
    physical_chunk_determinism_violation_count: AtomicU64,
    milestone_6_proof_only_prepare_count: AtomicU64,
    milestone_6_on_demand_materialize_count: AtomicU64,
    milestone_6_policy_eager_resolution_count: AtomicU64,
    milestone_6_policy_eager_publish_count: AtomicU64,
    milestone_6_policy_eager_reuse_existing_count: AtomicU64,
    milestone_7_layout_reference_admission_count: AtomicU64,
    milestone_9_physical_chunk_reference_admission_count: AtomicU64,
    bulk_program_plan_count: AtomicU64,
    bulk_source_manifest_member_count: AtomicU64,
    bulk_source_manifest_stream_pass_count: AtomicU64,
    bulk_transform_partition_count: AtomicU64,
    bulk_chunk_plan_count: AtomicU64,
    bulk_chunk_execute_count: AtomicU64,
    bulk_checkpoint_write_count: AtomicU64,
    bulk_chunk_witness_write_count: AtomicU64,
    bulk_resume_index_lookup_count: AtomicU64,
    bulk_chunk_resume_count: AtomicU64,
    bulk_chunk_commit_count: AtomicU64,
    bulk_chunk_width_units: AtomicU64,
    bulk_peak_in_flight_memory_units: AtomicU64,
    bulk_fallback_path_count: AtomicU64,
    bulk_fallback_breadth_units: AtomicU64,
}

impl StoreCounters {
    pub fn record_durable_mode_selection(&self) {
        self.durable_mode_selection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_embedded_mode_selection(&self) {
        self.embedded_mode_selection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_hosted_runtime_start(&self) {
        self.hosted_runtime_start_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_hosted_runtime_stop(&self) {
        self.hosted_runtime_stop_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_external_commit_intake(&self) {
        self.external_commit_intake_count
            .fetch_add(1, Ordering::Relaxed);
        self.cross_mode_canonical_boundary_reuse_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_external_checkpoint_intake(&self) {
        self.external_checkpoint_intake_count
            .fetch_add(1, Ordering::Relaxed);
    }

    #[cfg(test)]
    pub fn record_embedded_checkpoint_authority_rejection(&self) {
        self.embedded_checkpoint_authority_rejection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    #[cfg(test)]
    pub fn record_mode_misuse_rejection(&self) {
        self.mode_misuse_rejection_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_canonicalization(&self, metrics: CanonicalizationMetrics) {
        self.canonicalization_item_count
            .fetch_add(metrics.canonicalization_item_count, Ordering::Relaxed);
        self.canonicalization_duplicate_collapse_count.fetch_add(
            metrics.canonicalization_duplicate_collapse_count,
            Ordering::Relaxed,
        );
    }

    pub fn record_append(&self, parent_count: usize, digest_writes: u64, branch_head_writes: u64) {
        self.authoritative_commit_append_count
            .fetch_add(1, Ordering::Relaxed);
        self.commit_parent_record_write_count
            .fetch_add(parent_count as u64, Ordering::Relaxed);
        self.authoritative_digest_write_count
            .fetch_add(digest_writes, Ordering::Relaxed);
        self.branch_head_write_count
            .fetch_add(branch_head_writes, Ordering::Relaxed);
    }

    pub fn record_commit_support_publication(&self) {
        self.commit_support_publication_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_commit_support_summary_build(&self) {
        self.commit_support_summary_build_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_commit_support_publication_gap(&self) {
        self.commit_support_publication_gap_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_schema_boundary_fetch(&self, index_lookups: u64, rows_read: u64) {
        self.schema_boundary_fetch_count
            .fetch_add(1, Ordering::Relaxed);
        self.schema_boundary_index_lookup_count
            .fetch_add(index_lookups, Ordering::Relaxed);
        self.schema_boundary_rows_read
            .fetch_add(rows_read, Ordering::Relaxed);
        self.schema_boundary_resolution_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_lineage_lookup(&self, identity_lookups: u64, event_rows_read: u64) {
        self.lineage_lookup_count.fetch_add(1, Ordering::Relaxed);
        self.lineage_identity_lookup_count
            .fetch_add(identity_lookups, Ordering::Relaxed);
        self.lineage_event_rows_read
            .fetch_add(event_rows_read, Ordering::Relaxed);
        self.lineage_resolution_breadth
            .fetch_add(event_rows_read, Ordering::Relaxed);
    }

    pub fn record_cursor_resume(&self, support_rows_read: u64, step_count: u64) {
        self.cursor_resume_count.fetch_add(1, Ordering::Relaxed);
        self.cursor_resume_support_rows_read
            .fetch_add(support_rows_read, Ordering::Relaxed);
        self.cursor_resume_step_count
            .fetch_add(step_count, Ordering::Relaxed);
    }

    pub fn record_cursor_identity_lookup(&self) {
        self.cursor_identity_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cursor_ack(&self) {
        self.cursor_ack_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cursor_equivalence_reject(&self) {
        self.cursor_equivalence_reject_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cursor_regression_reject(&self) {
        self.cursor_regression_reject_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_subscriber_checkpoint_write(&self) {
        self.subscriber_checkpoint_write_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_embedded_checkpoint_fetch(&self, basis_reads: u64) {
        self.embedded_checkpoint_fetch_count
            .fetch_add(1, Ordering::Relaxed);
        self.embedded_checkpoint_index_lookup_count
            .fetch_add(1, Ordering::Relaxed);
        self.embedded_checkpoint_basis_read_count
            .fetch_add(basis_reads, Ordering::Relaxed);
    }

    pub fn record_checkpoint_shape_reject(&self) {
        self.checkpoint_shape_reject_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_support_artifact_recovery_gap(&self, count: u64) {
        self.support_artifact_recovery_gap_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_state_delta_apply(&self, touched_families: u64, touched_records: u64) {
        self.state_delta_apply_count.fetch_add(1, Ordering::Relaxed);
        self.state_delta_touched_family_count
            .fetch_add(touched_families, Ordering::Relaxed);
        self.state_delta_touched_record_count
            .fetch_add(touched_records, Ordering::Relaxed);
    }

    pub fn record_fetch_verification(&self, success: bool) {
        self.authoritative_commit_fetch_count
            .fetch_add(1, Ordering::Relaxed);
        self.authoritative_fetch_verification_count
            .fetch_add(1, Ordering::Relaxed);
        if !success {
            self.authoritative_fetch_verification_failure_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_wal_append(&self) {
        self.wal_record_append_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_wal_scan(&self, count: usize) {
        self.wal_record_scan_count
            .fetch_add(count as u64, Ordering::Relaxed);
    }

    pub fn record_wal_decode_failure(&self) {
        self.wal_record_decode_failure_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_mutation_admit(&self) {
        self.durable_mutation_admit_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_commit_acknowledged(&self) {
        self.durable_commit_acknowledged_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_commit_recovered(&self) {
        self.durable_commit_recovered_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_commit_duplicate_suppressed(&self) {
        self.durable_commit_duplicate_suppression_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_commit_unacknowledged_discard(&self) {
        self.durable_commit_unacknowledged_discard_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_requires_full_rebuild(&self) {
        self.recovery_requires_full_rebuild_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_failure(&self) {
        self.recovery_failure_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_barrier_verified(&self) {
        self.durable_barrier_verified_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_durable_ack_barrier_violation(&self) {
        self.durable_ack_barrier_violation_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_source_precedence_resolution(&self) {
        self.recovery_source_precedence_resolution_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_source_precedence_fallback(&self) {
        self.recovery_source_precedence_fallback_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_quiescent_restart(&self) {
        self.recovery_quiescent_restart_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_non_quiescent_restart(&self) {
        self.recovery_non_quiescent_restart_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_recovery_quarantine(&self) {
        self.recovery_quarantine_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_snapshot_capture(&self, record_count: usize, byte_count: usize) {
        self.snapshot_capture_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot_capture_record_count
            .fetch_add(record_count as u64, Ordering::Relaxed);
        self.snapshot_capture_byte_count
            .fetch_add(byte_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_read(
        &self,
        record_count: usize,
        tail_commit_count: usize,
        tail_replay_count: usize,
    ) {
        self.snapshot_read_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot_read_record_count
            .fetch_add(record_count as u64, Ordering::Relaxed);
        self.snapshot_read_tail_commit_count
            .fetch_add(tail_commit_count as u64, Ordering::Relaxed);
        self.snapshot_read_tail_replay_count
            .fetch_add(tail_replay_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_restore(&self, tail_commit_count: usize, tail_replay_count: usize) {
        self.snapshot_restore_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot_restore_tail_commit_count
            .fetch_add(tail_commit_count as u64, Ordering::Relaxed);
        self.snapshot_restore_tail_replay_count
            .fetch_add(tail_replay_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_rebuild(&self, record_count: usize) {
        self.snapshot_rebuild_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot_rebuild_record_count
            .fetch_add(record_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_integrity_failure(&self) {
        self.snapshot_integrity_failure_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_snapshot_basis_mismatch(&self) {
        self.snapshot_basis_mismatch_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_create(&self) {
        self.branch_create_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_base_reuse(&self) {
        self.branch_base_reuse_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_delta_read(
        &self,
        layers_traversed: usize,
        read_record_count: usize,
        replay_commit_count: usize,
        used_authority_replay_fallback: bool,
    ) {
        self.branch_delta_read_count.fetch_add(1, Ordering::Relaxed);
        self.branch_delta_layers_traversed_count
            .fetch_add(layers_traversed as u64, Ordering::Relaxed);
        self.branch_delta_read_record_count
            .fetch_add(read_record_count as u64, Ordering::Relaxed);
        self.branch_delta_replay_commit_count
            .fetch_add(replay_commit_count as u64, Ordering::Relaxed);
        if used_authority_replay_fallback {
            self.branch_delta_authority_replay_fallback_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_branch_delta_merge_path_search(&self) {
        self.branch_delta_merge_path_search_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_delta_integrity_failure(&self) {
        self.branch_delta_integrity_failure_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_branch_delta_rewrite(
        &self,
        replaced_layer_count: usize,
        rewrite_record_count: usize,
        used_hidden_full_stack_rewrite: bool,
    ) {
        self.branch_delta_rewrite_count
            .fetch_add(1, Ordering::Relaxed);
        self.branch_delta_rewrite_layers_replaced_count
            .fetch_add(replaced_layer_count as u64, Ordering::Relaxed);
        self.branch_delta_rewrite_record_count
            .fetch_add(rewrite_record_count as u64, Ordering::Relaxed);
        if used_hidden_full_stack_rewrite {
            self.branch_delta_hidden_full_stack_rewrite_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_branch_delta_rebuild(&self, rebuilt_record_count: usize) {
        self.branch_delta_rebuild_count
            .fetch_add(1, Ordering::Relaxed);
        self.branch_delta_rebuild_record_count
            .fetch_add(rebuilt_record_count as u64, Ordering::Relaxed);
    }

    pub fn record_aspect_layout_plan(
        &self,
        admitted: bool,
        fallback: bool,
        rejected: bool,
        slice_read_count: usize,
        block_decode_count: usize,
        control_replay_breadth: usize,
    ) {
        self.aspect_layout_plan_count
            .fetch_add(1, Ordering::Relaxed);
        if admitted {
            self.aspect_layout_admitted_count
                .fetch_add(1, Ordering::Relaxed);
        }
        if fallback {
            self.aspect_layout_fallback_count
                .fetch_add(1, Ordering::Relaxed);
        }
        if rejected {
            self.aspect_layout_rejected_count
                .fetch_add(1, Ordering::Relaxed);
        }
        self.aspect_layout_slice_read_count
            .fetch_add(slice_read_count as u64, Ordering::Relaxed);
        self.aspect_layout_block_decode_count
            .fetch_add(block_decode_count as u64, Ordering::Relaxed);
        self.aspect_layout_control_replay_breadth
            .fetch_add(control_replay_breadth as u64, Ordering::Relaxed);
    }

    pub fn record_structural_block_lookup(&self, hit: bool) {
        self.structural_block_lookup_count
            .fetch_add(1, Ordering::Relaxed);
        if hit {
            self.structural_block_reuse_hit_count
                .fetch_add(1, Ordering::Relaxed);
        } else {
            self.structural_block_reuse_miss_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_structural_block_reuse_admission(&self) {
        self.structural_block_reuse_admission_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_chunk_model_freeze(&self) {
        self.chunk_model_freeze_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_physical_chunk_export(&self, chunk_width: u64) {
        self.physical_chunk_export_count
            .fetch_add(1, Ordering::Relaxed);
        self.physical_chunk_width_count
            .fetch_add(chunk_width, Ordering::Relaxed);
    }

    pub fn record_physical_chunk_determinism_violation(&self) {
        self.physical_chunk_determinism_violation_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_6_proof_only_prepare(&self) {
        self.milestone_6_proof_only_prepare_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_6_on_demand_materialize(&self) {
        self.milestone_6_on_demand_materialize_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_6_policy_eager_resolution(&self) {
        self.milestone_6_policy_eager_resolution_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_6_policy_eager_publish(&self) {
        self.milestone_6_policy_eager_publish_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_6_policy_eager_reuse_existing(&self) {
        self.milestone_6_policy_eager_reuse_existing_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_7_layout_reference_admission(&self) {
        self.milestone_7_layout_reference_admission_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_milestone_9_physical_chunk_reference_admission(&self) {
        self.milestone_9_physical_chunk_reference_admission_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_source_manifest(&self, member_count: u64, stream_pass_count: u64) {
        self.bulk_program_plan_count.fetch_add(1, Ordering::Relaxed);
        self.bulk_source_manifest_member_count
            .fetch_add(member_count, Ordering::Relaxed);
        self.bulk_source_manifest_stream_pass_count
            .fetch_add(stream_pass_count, Ordering::Relaxed);
    }

    pub fn record_bulk_chunk_plan(&self, chunk_count: u64) {
        self.bulk_chunk_plan_count
            .fetch_add(chunk_count, Ordering::Relaxed);
    }

    pub fn record_bulk_transform_partition(&self, partition_count: u64) {
        self.bulk_transform_partition_count
            .fetch_add(partition_count, Ordering::Relaxed);
    }

    pub fn record_bulk_checkpoint_write(&self) {
        self.bulk_checkpoint_write_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_chunk_witness_write(&self) {
        self.bulk_chunk_witness_write_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_resume_index_lookup(&self) {
        self.bulk_resume_index_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_chunk_resume(&self) {
        self.bulk_chunk_resume_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_chunk_commit(&self) {
        self.bulk_chunk_commit_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_bulk_chunk_execute(
        &self,
        width_units: u64,
        memory_units: u64,
        fallback_breadth_units: u64,
        used_fallback_path: bool,
    ) {
        self.bulk_chunk_execute_count
            .fetch_add(1, Ordering::Relaxed);
        self.bulk_chunk_width_units
            .fetch_add(width_units, Ordering::Relaxed);

        let mut current_peak = self
            .bulk_peak_in_flight_memory_units
            .load(Ordering::Relaxed);
        while memory_units > current_peak {
            match self.bulk_peak_in_flight_memory_units.compare_exchange(
                current_peak,
                memory_units,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => current_peak = observed,
            }
        }

        if used_fallback_path {
            self.bulk_fallback_path_count
                .fetch_add(1, Ordering::Relaxed);
            self.bulk_fallback_breadth_units
                .fetch_add(fallback_breadth_units, Ordering::Relaxed);
        }
    }

    pub fn snapshot(&self) -> StoreCounterSnapshot {
        StoreCounterSnapshot {
            durable_mode_selection_count: self.durable_mode_selection_count.load(Ordering::Relaxed),
            embedded_mode_selection_count: self
                .embedded_mode_selection_count
                .load(Ordering::Relaxed),
            absent_mode_selection_count: self.absent_mode_selection_count.load(Ordering::Relaxed),
            hosted_runtime_start_count: self.hosted_runtime_start_count.load(Ordering::Relaxed),
            hosted_runtime_stop_count: self.hosted_runtime_stop_count.load(Ordering::Relaxed),
            external_commit_intake_count: self.external_commit_intake_count.load(Ordering::Relaxed),
            external_checkpoint_intake_count: self
                .external_checkpoint_intake_count
                .load(Ordering::Relaxed),
            embedded_checkpoint_authority_rejection_count: self
                .embedded_checkpoint_authority_rejection_count
                .load(Ordering::Relaxed),
            cross_mode_canonical_boundary_reuse_count: self
                .cross_mode_canonical_boundary_reuse_count
                .load(Ordering::Relaxed),
            mode_misuse_rejection_count: self.mode_misuse_rejection_count.load(Ordering::Relaxed),
            absent_mode_store_touch_count: self
                .absent_mode_store_touch_count
                .load(Ordering::Relaxed),
            authoritative_commit_append_count: self
                .authoritative_commit_append_count
                .load(Ordering::Relaxed),
            authoritative_commit_fetch_count: self
                .authoritative_commit_fetch_count
                .load(Ordering::Relaxed),
            commit_parent_record_write_count: self
                .commit_parent_record_write_count
                .load(Ordering::Relaxed),
            branch_head_write_count: self.branch_head_write_count.load(Ordering::Relaxed),
            authoritative_digest_write_count: self
                .authoritative_digest_write_count
                .load(Ordering::Relaxed),
            commit_support_publication_count: self
                .commit_support_publication_count
                .load(Ordering::Relaxed),
            commit_support_publication_gap_count: self
                .commit_support_publication_gap_count
                .load(Ordering::Relaxed),
            commit_support_summary_build_count: self
                .commit_support_summary_build_count
                .load(Ordering::Relaxed),
            schema_boundary_fetch_count: self.schema_boundary_fetch_count.load(Ordering::Relaxed),
            schema_boundary_index_lookup_count: self
                .schema_boundary_index_lookup_count
                .load(Ordering::Relaxed),
            schema_boundary_rows_read: self.schema_boundary_rows_read.load(Ordering::Relaxed),
            schema_boundary_resolution_count: self
                .schema_boundary_resolution_count
                .load(Ordering::Relaxed),
            lineage_lookup_count: self.lineage_lookup_count.load(Ordering::Relaxed),
            lineage_identity_lookup_count: self
                .lineage_identity_lookup_count
                .load(Ordering::Relaxed),
            lineage_event_rows_read: self.lineage_event_rows_read.load(Ordering::Relaxed),
            lineage_resolution_breadth: self.lineage_resolution_breadth.load(Ordering::Relaxed),
            cursor_resume_count: self.cursor_resume_count.load(Ordering::Relaxed),
            cursor_identity_lookup_count: self.cursor_identity_lookup_count.load(Ordering::Relaxed),
            cursor_resume_support_rows_read: self
                .cursor_resume_support_rows_read
                .load(Ordering::Relaxed),
            cursor_resume_step_count: self.cursor_resume_step_count.load(Ordering::Relaxed),
            cursor_ack_count: self.cursor_ack_count.load(Ordering::Relaxed),
            cursor_equivalence_reject_count: self
                .cursor_equivalence_reject_count
                .load(Ordering::Relaxed),
            cursor_regression_reject_count: self
                .cursor_regression_reject_count
                .load(Ordering::Relaxed),
            subscriber_checkpoint_write_count: self
                .subscriber_checkpoint_write_count
                .load(Ordering::Relaxed),
            embedded_checkpoint_fetch_count: self
                .embedded_checkpoint_fetch_count
                .load(Ordering::Relaxed),
            embedded_checkpoint_index_lookup_count: self
                .embedded_checkpoint_index_lookup_count
                .load(Ordering::Relaxed),
            embedded_checkpoint_basis_read_count: self
                .embedded_checkpoint_basis_read_count
                .load(Ordering::Relaxed),
            checkpoint_shape_reject_count: self
                .checkpoint_shape_reject_count
                .load(Ordering::Relaxed),
            support_artifact_recovery_gap_count: self
                .support_artifact_recovery_gap_count
                .load(Ordering::Relaxed),
            state_delta_apply_count: self.state_delta_apply_count.load(Ordering::Relaxed),
            state_delta_touched_family_count: self
                .state_delta_touched_family_count
                .load(Ordering::Relaxed),
            state_delta_touched_record_count: self
                .state_delta_touched_record_count
                .load(Ordering::Relaxed),
            state_clone_fallback_count: self.state_clone_fallback_count.load(Ordering::Relaxed),
            canonicalization_item_count: self.canonicalization_item_count.load(Ordering::Relaxed),
            canonicalization_duplicate_collapse_count: self
                .canonicalization_duplicate_collapse_count
                .load(Ordering::Relaxed),
            authoritative_fetch_verification_count: self
                .authoritative_fetch_verification_count
                .load(Ordering::Relaxed),
            authoritative_fetch_verification_failure_count: self
                .authoritative_fetch_verification_failure_count
                .load(Ordering::Relaxed),
            wal_record_append_count: self.wal_record_append_count.load(Ordering::Relaxed),
            wal_record_scan_count: self.wal_record_scan_count.load(Ordering::Relaxed),
            wal_record_decode_failure_count: self
                .wal_record_decode_failure_count
                .load(Ordering::Relaxed),
            durable_mutation_admit_count: self.durable_mutation_admit_count.load(Ordering::Relaxed),
            durable_commit_acknowledged_count: self
                .durable_commit_acknowledged_count
                .load(Ordering::Relaxed),
            durable_commit_recovered_count: self
                .durable_commit_recovered_count
                .load(Ordering::Relaxed),
            durable_commit_duplicate_suppression_count: self
                .durable_commit_duplicate_suppression_count
                .load(Ordering::Relaxed),
            durable_commit_unacknowledged_discard_count: self
                .durable_commit_unacknowledged_discard_count
                .load(Ordering::Relaxed),
            recovery_requires_full_rebuild_count: self
                .recovery_requires_full_rebuild_count
                .load(Ordering::Relaxed),
            recovery_failure_count: self.recovery_failure_count.load(Ordering::Relaxed),
            durable_frame_scan_count: self.durable_frame_scan_count.load(Ordering::Relaxed),
            durable_frame_reject_count: self.durable_frame_reject_count.load(Ordering::Relaxed),
            durable_truncated_tail_count: self.durable_truncated_tail_count.load(Ordering::Relaxed),
            durable_torn_write_count: self.durable_torn_write_count.load(Ordering::Relaxed),
            durable_barrier_verified_count: self
                .durable_barrier_verified_count
                .load(Ordering::Relaxed),
            durable_ack_barrier_violation_count: self
                .durable_ack_barrier_violation_count
                .load(Ordering::Relaxed),
            recovery_source_precedence_resolution_count: self
                .recovery_source_precedence_resolution_count
                .load(Ordering::Relaxed),
            recovery_source_precedence_fallback_count: self
                .recovery_source_precedence_fallback_count
                .load(Ordering::Relaxed),
            recovery_quiescent_restart_count: self
                .recovery_quiescent_restart_count
                .load(Ordering::Relaxed),
            recovery_non_quiescent_restart_count: self
                .recovery_non_quiescent_restart_count
                .load(Ordering::Relaxed),
            recovery_quarantine_count: self.recovery_quarantine_count.load(Ordering::Relaxed),
            recovery_salvage_count: self.recovery_salvage_count.load(Ordering::Relaxed),
            interrupted_maintenance_recovery_count: self
                .interrupted_maintenance_recovery_count
                .load(Ordering::Relaxed),
            backup_restore_compatibility_reject_count: self
                .backup_restore_compatibility_reject_count
                .load(Ordering::Relaxed),
            snapshot_capture_count: self.snapshot_capture_count.load(Ordering::Relaxed),
            snapshot_capture_record_count: self
                .snapshot_capture_record_count
                .load(Ordering::Relaxed),
            snapshot_capture_byte_count: self.snapshot_capture_byte_count.load(Ordering::Relaxed),
            snapshot_read_count: self.snapshot_read_count.load(Ordering::Relaxed),
            snapshot_read_record_count: self.snapshot_read_record_count.load(Ordering::Relaxed),
            snapshot_read_tail_commit_count: self
                .snapshot_read_tail_commit_count
                .load(Ordering::Relaxed),
            snapshot_read_tail_replay_count: self
                .snapshot_read_tail_replay_count
                .load(Ordering::Relaxed),
            snapshot_restore_count: self.snapshot_restore_count.load(Ordering::Relaxed),
            snapshot_restore_tail_commit_count: self
                .snapshot_restore_tail_commit_count
                .load(Ordering::Relaxed),
            snapshot_restore_tail_replay_count: self
                .snapshot_restore_tail_replay_count
                .load(Ordering::Relaxed),
            snapshot_rebuild_count: self.snapshot_rebuild_count.load(Ordering::Relaxed),
            snapshot_rebuild_record_count: self
                .snapshot_rebuild_record_count
                .load(Ordering::Relaxed),
            snapshot_integrity_failure_count: self
                .snapshot_integrity_failure_count
                .load(Ordering::Relaxed),
            snapshot_basis_mismatch_count: self
                .snapshot_basis_mismatch_count
                .load(Ordering::Relaxed),
            branch_create_count: self.branch_create_count.load(Ordering::Relaxed),
            branch_base_reuse_count: self.branch_base_reuse_count.load(Ordering::Relaxed),
            branch_base_copy_count: self.branch_base_copy_count.load(Ordering::Relaxed),
            branch_hidden_full_base_materialization_count: self
                .branch_hidden_full_base_materialization_count
                .load(Ordering::Relaxed),
            branch_delta_read_count: self.branch_delta_read_count.load(Ordering::Relaxed),
            branch_delta_layers_traversed_count: self
                .branch_delta_layers_traversed_count
                .load(Ordering::Relaxed),
            branch_delta_read_record_count: self
                .branch_delta_read_record_count
                .load(Ordering::Relaxed),
            branch_delta_replay_commit_count: self
                .branch_delta_replay_commit_count
                .load(Ordering::Relaxed),
            branch_delta_authority_replay_fallback_count: self
                .branch_delta_authority_replay_fallback_count
                .load(Ordering::Relaxed),
            branch_delta_rewrite_count: self.branch_delta_rewrite_count.load(Ordering::Relaxed),
            branch_delta_rewrite_layers_replaced_count: self
                .branch_delta_rewrite_layers_replaced_count
                .load(Ordering::Relaxed),
            branch_delta_rewrite_record_count: self
                .branch_delta_rewrite_record_count
                .load(Ordering::Relaxed),
            branch_delta_hidden_full_stack_rewrite_count: self
                .branch_delta_hidden_full_stack_rewrite_count
                .load(Ordering::Relaxed),
            branch_delta_merge_path_search_count: self
                .branch_delta_merge_path_search_count
                .load(Ordering::Relaxed),
            branch_delta_rebuild_count: self.branch_delta_rebuild_count.load(Ordering::Relaxed),
            branch_delta_rebuild_record_count: self
                .branch_delta_rebuild_record_count
                .load(Ordering::Relaxed),
            branch_delta_integrity_failure_count: self
                .branch_delta_integrity_failure_count
                .load(Ordering::Relaxed),
            concurrent_artifact_boundary_rejection_count: self
                .concurrent_artifact_boundary_rejection_count
                .load(Ordering::Relaxed),
            aspect_layout_plan_count: self.aspect_layout_plan_count.load(Ordering::Relaxed),
            aspect_layout_admitted_count: self.aspect_layout_admitted_count.load(Ordering::Relaxed),
            aspect_layout_fallback_count: self.aspect_layout_fallback_count.load(Ordering::Relaxed),
            aspect_layout_rejected_count: self.aspect_layout_rejected_count.load(Ordering::Relaxed),
            aspect_layout_slice_read_count: self
                .aspect_layout_slice_read_count
                .load(Ordering::Relaxed),
            aspect_layout_block_decode_count: self
                .aspect_layout_block_decode_count
                .load(Ordering::Relaxed),
            aspect_layout_control_replay_breadth: self
                .aspect_layout_control_replay_breadth
                .load(Ordering::Relaxed),
            aspect_layout_whole_state_fallback_count: self
                .aspect_layout_whole_state_fallback_count
                .load(Ordering::Relaxed),
            structural_block_lookup_count: self
                .structural_block_lookup_count
                .load(Ordering::Relaxed),
            structural_block_reuse_admission_count: self
                .structural_block_reuse_admission_count
                .load(Ordering::Relaxed),
            structural_block_reuse_hit_count: self
                .structural_block_reuse_hit_count
                .load(Ordering::Relaxed),
            structural_block_reuse_miss_count: self
                .structural_block_reuse_miss_count
                .load(Ordering::Relaxed),
            chunk_model_freeze_count: self.chunk_model_freeze_count.load(Ordering::Relaxed),
            physical_chunk_export_count: self.physical_chunk_export_count.load(Ordering::Relaxed),
            physical_chunk_width_count: self.physical_chunk_width_count.load(Ordering::Relaxed),
            physical_chunk_determinism_violation_count: self
                .physical_chunk_determinism_violation_count
                .load(Ordering::Relaxed),
            milestone_6_proof_only_prepare_count: self
                .milestone_6_proof_only_prepare_count
                .load(Ordering::Relaxed),
            milestone_6_on_demand_materialize_count: self
                .milestone_6_on_demand_materialize_count
                .load(Ordering::Relaxed),
            milestone_6_policy_eager_resolution_count: self
                .milestone_6_policy_eager_resolution_count
                .load(Ordering::Relaxed),
            milestone_6_policy_eager_publish_count: self
                .milestone_6_policy_eager_publish_count
                .load(Ordering::Relaxed),
            milestone_6_policy_eager_reuse_existing_count: self
                .milestone_6_policy_eager_reuse_existing_count
                .load(Ordering::Relaxed),
            milestone_7_layout_reference_admission_count: self
                .milestone_7_layout_reference_admission_count
                .load(Ordering::Relaxed),
            milestone_9_physical_chunk_reference_admission_count: self
                .milestone_9_physical_chunk_reference_admission_count
                .load(Ordering::Relaxed),
            bulk_program_plan_count: self.bulk_program_plan_count.load(Ordering::Relaxed),
            bulk_source_manifest_member_count: self
                .bulk_source_manifest_member_count
                .load(Ordering::Relaxed),
            bulk_source_manifest_stream_pass_count: self
                .bulk_source_manifest_stream_pass_count
                .load(Ordering::Relaxed),
            bulk_transform_partition_count: self
                .bulk_transform_partition_count
                .load(Ordering::Relaxed),
            bulk_chunk_plan_count: self.bulk_chunk_plan_count.load(Ordering::Relaxed),
            bulk_chunk_execute_count: self.bulk_chunk_execute_count.load(Ordering::Relaxed),
            bulk_checkpoint_write_count: self.bulk_checkpoint_write_count.load(Ordering::Relaxed),
            bulk_chunk_witness_write_count: self
                .bulk_chunk_witness_write_count
                .load(Ordering::Relaxed),
            bulk_resume_index_lookup_count: self
                .bulk_resume_index_lookup_count
                .load(Ordering::Relaxed),
            bulk_chunk_resume_count: self.bulk_chunk_resume_count.load(Ordering::Relaxed),
            bulk_chunk_commit_count: self.bulk_chunk_commit_count.load(Ordering::Relaxed),
            bulk_chunk_width_units: self.bulk_chunk_width_units.load(Ordering::Relaxed),
            bulk_peak_in_flight_memory_units: self
                .bulk_peak_in_flight_memory_units
                .load(Ordering::Relaxed),
            bulk_fallback_path_count: self.bulk_fallback_path_count.load(Ordering::Relaxed),
            bulk_fallback_breadth_units: self.bulk_fallback_breadth_units.load(Ordering::Relaxed),
        }
    }
}
