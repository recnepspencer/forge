use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct SnapshotCounters {
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
    pub fn record_snapshot_capture(&self, record_count: usize, byte_count: usize) {
        self.snapshot.snapshot_capture_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot.snapshot_capture_record_count.fetch_add(record_count as u64, Ordering::Relaxed);
        self.snapshot.snapshot_capture_byte_count.fetch_add(byte_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_read(
        &self,
        record_count: usize,
        tail_commit_count: usize,
        tail_replay_count: usize,
    ) {
        self.snapshot.snapshot_read_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot.snapshot_read_record_count.fetch_add(record_count as u64, Ordering::Relaxed);
        self.snapshot.snapshot_read_tail_commit_count.fetch_add(tail_commit_count as u64, Ordering::Relaxed);
        self.snapshot.snapshot_read_tail_replay_count.fetch_add(tail_replay_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_restore(&self, tail_commit_count: usize, tail_replay_count: usize) {
        self.snapshot.snapshot_restore_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot.snapshot_restore_tail_commit_count.fetch_add(tail_commit_count as u64, Ordering::Relaxed);
        self.snapshot.snapshot_restore_tail_replay_count.fetch_add(tail_replay_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_rebuild(&self, record_count: usize) {
        self.snapshot.snapshot_rebuild_count.fetch_add(1, Ordering::Relaxed);
        self.snapshot.snapshot_rebuild_record_count.fetch_add(record_count as u64, Ordering::Relaxed);
    }

    pub fn record_snapshot_integrity_failure(&self) {
        self.snapshot.snapshot_integrity_failure_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_snapshot_basis_mismatch(&self) {
        self.snapshot.snapshot_basis_mismatch_count.fetch_add(1, Ordering::Relaxed);
    }
}

pub(super) fn write_snapshot(counters: &SnapshotCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }
    load!(snapshot_capture_count);
    load!(snapshot_capture_record_count);
    load!(snapshot_capture_byte_count);
    load!(snapshot_read_count);
    load!(snapshot_read_record_count);
    load!(snapshot_read_tail_commit_count);
    load!(snapshot_read_tail_replay_count);
    load!(snapshot_restore_count);
    load!(snapshot_restore_tail_commit_count);
    load!(snapshot_restore_tail_replay_count);
    load!(snapshot_rebuild_count);
    load!(snapshot_rebuild_record_count);
    load!(snapshot_integrity_failure_count);
    load!(snapshot_basis_mismatch_count);
}
