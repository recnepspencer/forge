use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct DurabilityCounters {
    state_delta_apply_count: AtomicU64,
    state_delta_touched_family_count: AtomicU64,
    state_delta_touched_record_count: AtomicU64,
    state_clone_fallback_count: AtomicU64,
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
}

impl StoreCounters {
    pub fn record_state_delta_apply(&self, touched_families: u64, touched_records: u64) {
        self.durability.state_delta_apply_count.fetch_add(1, Ordering::Relaxed);
        self.durability.state_delta_touched_family_count.fetch_add(touched_families, Ordering::Relaxed);
        self.durability.state_delta_touched_record_count.fetch_add(touched_records, Ordering::Relaxed);
    }

    pub fn record_wal_append(&self) { self.durability.wal_record_append_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_wal_scan(&self, count: usize) { self.durability.wal_record_scan_count.fetch_add(count as u64, Ordering::Relaxed); }
    pub fn record_wal_decode_failure(&self) { self.durability.wal_record_decode_failure_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_mutation_admit(&self) { self.durability.durable_mutation_admit_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_commit_acknowledged(&self) { self.durability.durable_commit_acknowledged_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_commit_recovered(&self) { self.durability.durable_commit_recovered_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_commit_duplicate_suppressed(&self) { self.durability.durable_commit_duplicate_suppression_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_commit_unacknowledged_discard(&self) { self.durability.durable_commit_unacknowledged_discard_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_requires_full_rebuild(&self) { self.durability.recovery_requires_full_rebuild_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_failure(&self) { self.durability.recovery_failure_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_barrier_verified(&self) { self.durability.durable_barrier_verified_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_durable_ack_barrier_violation(&self) { self.durability.durable_ack_barrier_violation_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_source_precedence_resolution(&self) { self.durability.recovery_source_precedence_resolution_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_source_precedence_fallback(&self) { self.durability.recovery_source_precedence_fallback_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_quiescent_restart(&self) { self.durability.recovery_quiescent_restart_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_non_quiescent_restart(&self) { self.durability.recovery_non_quiescent_restart_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_recovery_quarantine(&self) { self.durability.recovery_quarantine_count.fetch_add(1, Ordering::Relaxed); }
}

pub(super) fn write_snapshot(counters: &DurabilityCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }
    load!(state_delta_apply_count);
    load!(state_delta_touched_family_count);
    load!(state_delta_touched_record_count);
    load!(state_clone_fallback_count);
    load!(wal_record_append_count);
    load!(wal_record_scan_count);
    load!(wal_record_decode_failure_count);
    load!(durable_mutation_admit_count);
    load!(durable_commit_acknowledged_count);
    load!(durable_commit_recovered_count);
    load!(durable_commit_duplicate_suppression_count);
    load!(durable_commit_unacknowledged_discard_count);
    load!(recovery_requires_full_rebuild_count);
    load!(recovery_failure_count);
    load!(durable_frame_scan_count);
    load!(durable_frame_reject_count);
    load!(durable_truncated_tail_count);
    load!(durable_torn_write_count);
    load!(durable_barrier_verified_count);
    load!(durable_ack_barrier_violation_count);
    load!(recovery_source_precedence_resolution_count);
    load!(recovery_source_precedence_fallback_count);
    load!(recovery_quiescent_restart_count);
    load!(recovery_non_quiescent_restart_count);
    load!(recovery_quarantine_count);
    load!(recovery_salvage_count);
    load!(interrupted_maintenance_recovery_count);
    load!(backup_restore_compatibility_reject_count);
}
