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

    pub fn record_embedded_checkpoint_authority_rejection(&self) {
        self.embedded_checkpoint_authority_rejection_count
            .fetch_add(1, Ordering::Relaxed);
    }

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
        }
    }
}
