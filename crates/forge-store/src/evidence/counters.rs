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
        }
    }
}
