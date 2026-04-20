use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct AuthorityCounters {
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
    authoritative_fetch_verification_count: AtomicU64,
    authoritative_fetch_verification_failure_count: AtomicU64,
}

impl StoreCounters {
    pub fn record_append(&self, parent_count: usize, digest_writes: u64, branch_head_writes: u64) {
        self.authority
            .authoritative_commit_append_count
            .fetch_add(1, Ordering::Relaxed);
        self.authority
            .commit_parent_record_write_count
            .fetch_add(parent_count as u64, Ordering::Relaxed);
        self.authority
            .authoritative_digest_write_count
            .fetch_add(digest_writes, Ordering::Relaxed);
        self.authority
            .branch_head_write_count
            .fetch_add(branch_head_writes, Ordering::Relaxed);
    }

    pub fn record_commit_support_publication(&self) {
        self.authority
            .commit_support_publication_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_commit_support_summary_build(&self) {
        self.authority
            .commit_support_summary_build_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_commit_support_publication_gap(&self) {
        self.authority
            .commit_support_publication_gap_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_schema_boundary_fetch(&self, index_lookups: u64, rows_read: u64) {
        self.authority
            .schema_boundary_fetch_count
            .fetch_add(1, Ordering::Relaxed);
        self.authority
            .schema_boundary_index_lookup_count
            .fetch_add(index_lookups, Ordering::Relaxed);
        self.authority
            .schema_boundary_rows_read
            .fetch_add(rows_read, Ordering::Relaxed);
        self.authority
            .schema_boundary_resolution_count
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_lineage_lookup(&self, identity_lookups: u64, event_rows_read: u64) {
        self.authority.lineage_lookup_count.fetch_add(1, Ordering::Relaxed);
        self.authority
            .lineage_identity_lookup_count
            .fetch_add(identity_lookups, Ordering::Relaxed);
        self.authority
            .lineage_event_rows_read
            .fetch_add(event_rows_read, Ordering::Relaxed);
        self.authority
            .lineage_resolution_breadth
            .fetch_add(event_rows_read, Ordering::Relaxed);
    }

    pub fn record_fetch_verification(&self, success: bool) {
        self.authority
            .authoritative_commit_fetch_count
            .fetch_add(1, Ordering::Relaxed);
        self.authority
            .authoritative_fetch_verification_count
            .fetch_add(1, Ordering::Relaxed);
        if !success {
            self.authority
                .authoritative_fetch_verification_failure_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub(super) fn write_snapshot(counters: &AuthorityCounters, snapshot: &mut StoreCounterSnapshot) {
    snapshot.authoritative_commit_append_count = counters
        .authoritative_commit_append_count
        .load(Ordering::Relaxed);
    snapshot.authoritative_commit_fetch_count = counters
        .authoritative_commit_fetch_count
        .load(Ordering::Relaxed);
    snapshot.commit_parent_record_write_count = counters
        .commit_parent_record_write_count
        .load(Ordering::Relaxed);
    snapshot.branch_head_write_count = counters.branch_head_write_count.load(Ordering::Relaxed);
    snapshot.authoritative_digest_write_count = counters
        .authoritative_digest_write_count
        .load(Ordering::Relaxed);
    snapshot.commit_support_publication_count = counters
        .commit_support_publication_count
        .load(Ordering::Relaxed);
    snapshot.commit_support_publication_gap_count = counters
        .commit_support_publication_gap_count
        .load(Ordering::Relaxed);
    snapshot.commit_support_summary_build_count = counters
        .commit_support_summary_build_count
        .load(Ordering::Relaxed);
    snapshot.schema_boundary_fetch_count =
        counters.schema_boundary_fetch_count.load(Ordering::Relaxed);
    snapshot.schema_boundary_index_lookup_count = counters
        .schema_boundary_index_lookup_count
        .load(Ordering::Relaxed);
    snapshot.schema_boundary_rows_read =
        counters.schema_boundary_rows_read.load(Ordering::Relaxed);
    snapshot.schema_boundary_resolution_count = counters
        .schema_boundary_resolution_count
        .load(Ordering::Relaxed);
    snapshot.lineage_lookup_count = counters.lineage_lookup_count.load(Ordering::Relaxed);
    snapshot.lineage_identity_lookup_count = counters
        .lineage_identity_lookup_count
        .load(Ordering::Relaxed);
    snapshot.lineage_event_rows_read =
        counters.lineage_event_rows_read.load(Ordering::Relaxed);
    snapshot.lineage_resolution_breadth =
        counters.lineage_resolution_breadth.load(Ordering::Relaxed);
    snapshot.authoritative_fetch_verification_count = counters
        .authoritative_fetch_verification_count
        .load(Ordering::Relaxed);
    snapshot.authoritative_fetch_verification_failure_count = counters
        .authoritative_fetch_verification_failure_count
        .load(Ordering::Relaxed);
}
