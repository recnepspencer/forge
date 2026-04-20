use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct LiveQueryCounters {
    cursor_resume_count: AtomicU64,
    cursor_identity_lookup_count: AtomicU64,
    cursor_resume_support_rows_read: AtomicU64,
    cursor_resume_step_count: AtomicU64,
    cursor_ack_count: AtomicU64,
    cursor_equivalence_reject_count: AtomicU64,
    cursor_regression_reject_count: AtomicU64,
    stable_basis_lookup_count: AtomicU64,
    stable_basis_read_count: AtomicU64,
    stable_basis_support_rows_read: AtomicU64,
    stable_basis_scope_lookup_count: AtomicU64,
    stable_basis_fallback_count: AtomicU64,
    stable_basis_broadening_count: AtomicU64,
    continuation_batch_gap_count: AtomicU64,
    continuation_batch_duplicate_count: AtomicU64,
    continuation_plan_count: AtomicU64,
    continuation_cursor_identity_lookup_count: AtomicU64,
    continuation_checkpoint_lookup_count: AtomicU64,
    continuation_support_rows_read: AtomicU64,
    continuation_batch_count: AtomicU64,
    continuation_narrowed_item_count: AtomicU64,
    continuation_broadened_item_count: AtomicU64,
    continuation_step_count: AtomicU64,
    continuation_schema_mismatch_count: AtomicU64,
    continuation_scope_mismatch_count: AtomicU64,
    continuation_degraded_basis_count: AtomicU64,
    continuation_rejected_basis_count: AtomicU64,
    continuation_control_lane_fallback_count: AtomicU64,
    continuation_broadening_count: AtomicU64,
    continuation_parity_count: AtomicU64,
    continuation_illegal_acknowledgment_count: AtomicU64,
    subscriber_checkpoint_write_count: AtomicU64,
    embedded_checkpoint_fetch_count: AtomicU64,
    embedded_checkpoint_index_lookup_count: AtomicU64,
    embedded_checkpoint_basis_read_count: AtomicU64,
    checkpoint_shape_reject_count: AtomicU64,
    support_artifact_recovery_gap_count: AtomicU64,
}

impl StoreCounters {
    pub fn record_cursor_resume(&self, support_rows_read: u64, step_count: u64) {
        self.live_query.cursor_resume_count.fetch_add(1, Ordering::Relaxed);
        self.live_query
            .cursor_resume_support_rows_read
            .fetch_add(support_rows_read, Ordering::Relaxed);
        self.live_query
            .cursor_resume_step_count
            .fetch_add(step_count, Ordering::Relaxed);
    }

    pub fn record_cursor_identity_lookup(&self) {
        self.live_query
            .cursor_identity_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cursor_ack(&self) {
        self.live_query.cursor_ack_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cursor_equivalence_reject(&self) {
        self.live_query
            .cursor_equivalence_reject_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cursor_regression_reject(&self) {
        self.live_query
            .cursor_regression_reject_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_stable_basis_lookup(&self) {
        self.live_query
            .stable_basis_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_stable_basis_read(
        &self,
        support_rows_read: u64,
        scope_lookup_count: u64,
        used_fallback: bool,
    ) {
        self.live_query
            .stable_basis_read_count
            .fetch_add(1, Ordering::Relaxed);
        self.live_query
            .stable_basis_support_rows_read
            .fetch_add(support_rows_read, Ordering::Relaxed);
        self.live_query
            .stable_basis_scope_lookup_count
            .fetch_add(scope_lookup_count, Ordering::Relaxed);
        if used_fallback {
            self.live_query
                .stable_basis_fallback_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_stable_basis_broadening(&self) {
        self.live_query
            .stable_basis_broadening_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_continuation_plan(&self) {
        self.live_query
            .continuation_plan_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_continuation_identity_lookup(&self) {
        self.live_query
            .continuation_cursor_identity_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_continuation_checkpoint_lookup(&self) {
        self.live_query
            .continuation_checkpoint_lookup_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_continuation_batch(&self) {
        self.live_query
            .continuation_batch_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_continuation_batch_metrics(
        &self,
        support_rows_read: u64,
        narrowed_item_count: u64,
        broadened_item_count: u64,
        step_count: u64,
    ) {
        self.live_query
            .continuation_support_rows_read
            .fetch_add(support_rows_read, Ordering::Relaxed);
        self.live_query
            .continuation_narrowed_item_count
            .fetch_add(narrowed_item_count, Ordering::Relaxed);
        self.live_query
            .continuation_broadened_item_count
            .fetch_add(broadened_item_count, Ordering::Relaxed);
        self.live_query
            .continuation_step_count
            .fetch_add(step_count, Ordering::Relaxed);
    }

    pub fn record_continuation_schema_mismatch(&self) { self.live_query.continuation_schema_mismatch_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_scope_mismatch(&self) { self.live_query.continuation_scope_mismatch_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_degraded_basis(&self) { self.live_query.continuation_degraded_basis_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_rejected_basis(&self) { self.live_query.continuation_rejected_basis_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_control_lane_fallback(&self) { self.live_query.continuation_control_lane_fallback_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_batch_gap(&self) { self.live_query.continuation_batch_gap_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_batch_duplicate(&self) { self.live_query.continuation_batch_duplicate_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_broadening(&self) { self.live_query.continuation_broadening_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_parity(&self) { self.live_query.continuation_parity_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_continuation_illegal_acknowledgment(&self) { self.live_query.continuation_illegal_acknowledgment_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_subscriber_checkpoint_write(&self) { self.live_query.subscriber_checkpoint_write_count.fetch_add(1, Ordering::Relaxed); }

    pub fn record_embedded_checkpoint_fetch(&self, basis_reads: u64) {
        self.live_query
            .embedded_checkpoint_fetch_count
            .fetch_add(1, Ordering::Relaxed);
        self.live_query
            .embedded_checkpoint_index_lookup_count
            .fetch_add(1, Ordering::Relaxed);
        self.live_query
            .embedded_checkpoint_basis_read_count
            .fetch_add(basis_reads, Ordering::Relaxed);
    }

    pub fn record_checkpoint_shape_reject(&self) { self.live_query.checkpoint_shape_reject_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_support_artifact_recovery_gap(&self, count: u64) { self.live_query.support_artifact_recovery_gap_count.fetch_add(count, Ordering::Relaxed); }
}

pub(super) fn write_snapshot(counters: &LiveQueryCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }
    load!(cursor_resume_count);
    load!(cursor_identity_lookup_count);
    load!(cursor_resume_support_rows_read);
    load!(cursor_resume_step_count);
    load!(cursor_ack_count);
    load!(cursor_equivalence_reject_count);
    load!(cursor_regression_reject_count);
    load!(stable_basis_lookup_count);
    load!(stable_basis_read_count);
    load!(stable_basis_support_rows_read);
    load!(stable_basis_scope_lookup_count);
    load!(stable_basis_fallback_count);
    load!(stable_basis_broadening_count);
    load!(continuation_batch_gap_count);
    load!(continuation_batch_duplicate_count);
    load!(continuation_plan_count);
    load!(continuation_cursor_identity_lookup_count);
    load!(continuation_checkpoint_lookup_count);
    load!(continuation_support_rows_read);
    load!(continuation_batch_count);
    load!(continuation_narrowed_item_count);
    load!(continuation_broadened_item_count);
    load!(continuation_step_count);
    load!(continuation_schema_mismatch_count);
    load!(continuation_scope_mismatch_count);
    load!(continuation_degraded_basis_count);
    load!(continuation_rejected_basis_count);
    load!(continuation_control_lane_fallback_count);
    load!(continuation_broadening_count);
    load!(continuation_parity_count);
    load!(continuation_illegal_acknowledgment_count);
    load!(subscriber_checkpoint_write_count);
    load!(embedded_checkpoint_fetch_count);
    load!(embedded_checkpoint_index_lookup_count);
    load!(embedded_checkpoint_basis_read_count);
    load!(checkpoint_shape_reject_count);
    load!(support_artifact_recovery_gap_count);
}
