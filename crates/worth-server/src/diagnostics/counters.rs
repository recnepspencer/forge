use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct WorthServerCounters {
    registered_surface_families: AtomicU64,
    rejected_duplicate_surface_registrations: AtomicU64,
    registered_operation_families: AtomicU64,
    rejected_duplicate_operation_registrations: AtomicU64,
    serve_start_count: AtomicU64,
    product_sessions_created: AtomicU64,
    product_session_preview_creations: AtomicU64,
    product_session_mutation_creations: AtomicU64,
    product_session_lookups_attempted: AtomicU64,
    product_session_lookups_denied_missing: AtomicU64,
    product_session_lookups_denied_foreign: AtomicU64,
    product_session_lookups_denied_expired: AtomicU64,
    product_session_lookups_denied_closed: AtomicU64,
    product_session_lookups_denied_moved: AtomicU64,
    product_session_lookups_denied_preview_for_mutation: AtomicU64,
    product_session_closes_recorded: AtomicU64,
    product_result_artifacts_emitted: AtomicU64,
    product_result_bytes_emitted: AtomicU64,
    product_result_oversized_denials: AtomicU64,
    durable_product_mutation_attempts: AtomicU64,
    durable_product_basis_comparisons: AtomicU64,
    durable_product_commits: AtomicU64,
    durable_product_previously_committed: AtomicU64,
    durable_product_idempotency_conflicts: AtomicU64,
    durable_product_stale_bases: AtomicU64,
    durable_product_indeterminate: AtomicU64,
    durable_product_recovery_attempts: AtomicU64,
    durable_product_recovery_resolved: AtomicU64,
    durable_product_recovery_failed: AtomicU64,
}

impl WorthServerCounters {
    pub fn record_registered_surface_families(&self, count: usize) {
        self.registered_surface_families
            .store(count as u64, Ordering::Relaxed);
    }

    pub fn increment_rejected_duplicate_surface_registrations(&self) {
        self.rejected_duplicate_surface_registrations
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_registered_operation_families(&self, count: usize) {
        self.registered_operation_families
            .store(count as u64, Ordering::Relaxed);
    }

    pub fn increment_rejected_duplicate_operation_registrations(&self) {
        self.rejected_duplicate_operation_registrations
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_serve_start_count(&self) {
        self.serve_start_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_sessions_created(&self) {
        self.product_sessions_created
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_preview_creations(&self) {
        self.product_session_preview_creations
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_mutation_creations(&self) {
        self.product_session_mutation_creations
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookups_attempted(&self) {
        self.product_session_lookups_attempted
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookup_denied_missing(&self) {
        self.product_session_lookups_denied_missing
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookup_denied_foreign(&self) {
        self.product_session_lookups_denied_foreign
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookup_denied_expired(&self) {
        self.product_session_lookups_denied_expired
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookup_denied_closed(&self) {
        self.product_session_lookups_denied_closed
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookup_denied_moved(&self) {
        self.product_session_lookups_denied_moved
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_lookup_denied_preview_for_mutation(&self) {
        self.product_session_lookups_denied_preview_for_mutation
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_product_session_closes_recorded(&self) {
        self.product_session_closes_recorded
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_product_result_artifact(&self, byte_len: usize) {
        self.product_result_artifacts_emitted
            .fetch_add(1, Ordering::Relaxed);
        self.product_result_bytes_emitted
            .fetch_add(byte_len as u64, Ordering::Relaxed);
    }

    pub(crate) fn increment_product_result_oversized_denials(&self) {
        self.product_result_oversized_denials
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_mutation_attempts(&self) {
        self.durable_product_mutation_attempts
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn record_durable_product_basis_comparisons(&self, count: u64) {
        self.durable_product_basis_comparisons
            .fetch_add(count, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_commits(&self) {
        self.durable_product_commits.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_previously_committed(&self) {
        self.durable_product_previously_committed
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_idempotency_conflicts(&self) {
        self.durable_product_idempotency_conflicts
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_stale_bases(&self) {
        self.durable_product_stale_bases
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_indeterminate(&self) {
        self.durable_product_indeterminate
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_recovery_attempts(&self) {
        self.durable_product_recovery_attempts
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_recovery_resolved(&self) {
        self.durable_product_recovery_resolved
            .fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn increment_durable_product_recovery_failed(&self) {
        self.durable_product_recovery_failed
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> WorthServerCounterSnapshot {
        WorthServerCounterSnapshot {
            registered_surface_families: self.registered_surface_families.load(Ordering::Relaxed),
            rejected_duplicate_surface_registrations: self
                .rejected_duplicate_surface_registrations
                .load(Ordering::Relaxed),
            registered_operation_families: self
                .registered_operation_families
                .load(Ordering::Relaxed),
            rejected_duplicate_operation_registrations: self
                .rejected_duplicate_operation_registrations
                .load(Ordering::Relaxed),
            serve_start_count: self.serve_start_count.load(Ordering::Relaxed),
            product_sessions_created: self.product_sessions_created.load(Ordering::Relaxed),
            product_session_preview_creations: self
                .product_session_preview_creations
                .load(Ordering::Relaxed),
            product_session_mutation_creations: self
                .product_session_mutation_creations
                .load(Ordering::Relaxed),
            product_session_lookups_attempted: self
                .product_session_lookups_attempted
                .load(Ordering::Relaxed),
            product_session_lookups_denied_missing: self
                .product_session_lookups_denied_missing
                .load(Ordering::Relaxed),
            product_session_lookups_denied_foreign: self
                .product_session_lookups_denied_foreign
                .load(Ordering::Relaxed),
            product_session_lookups_denied_expired: self
                .product_session_lookups_denied_expired
                .load(Ordering::Relaxed),
            product_session_lookups_denied_closed: self
                .product_session_lookups_denied_closed
                .load(Ordering::Relaxed),
            product_session_lookups_denied_moved: self
                .product_session_lookups_denied_moved
                .load(Ordering::Relaxed),
            product_session_lookups_denied_preview_for_mutation: self
                .product_session_lookups_denied_preview_for_mutation
                .load(Ordering::Relaxed),
            product_session_closes_recorded: self
                .product_session_closes_recorded
                .load(Ordering::Relaxed),
            product_result_artifacts_emitted: self
                .product_result_artifacts_emitted
                .load(Ordering::Relaxed),
            product_result_bytes_emitted: self.product_result_bytes_emitted.load(Ordering::Relaxed),
            product_result_oversized_denials: self
                .product_result_oversized_denials
                .load(Ordering::Relaxed),
            durable_product_mutation_attempts: self
                .durable_product_mutation_attempts
                .load(Ordering::Relaxed),
            durable_product_basis_comparisons: self
                .durable_product_basis_comparisons
                .load(Ordering::Relaxed),
            durable_product_commits: self.durable_product_commits.load(Ordering::Relaxed),
            durable_product_previously_committed: self
                .durable_product_previously_committed
                .load(Ordering::Relaxed),
            durable_product_idempotency_conflicts: self
                .durable_product_idempotency_conflicts
                .load(Ordering::Relaxed),
            durable_product_stale_bases: self.durable_product_stale_bases.load(Ordering::Relaxed),
            durable_product_indeterminate: self
                .durable_product_indeterminate
                .load(Ordering::Relaxed),
            durable_product_recovery_attempts: self
                .durable_product_recovery_attempts
                .load(Ordering::Relaxed),
            durable_product_recovery_resolved: self
                .durable_product_recovery_resolved
                .load(Ordering::Relaxed),
            durable_product_recovery_failed: self
                .durable_product_recovery_failed
                .load(Ordering::Relaxed),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCounterSnapshot {
    pub registered_surface_families: u64,
    pub rejected_duplicate_surface_registrations: u64,
    pub registered_operation_families: u64,
    pub rejected_duplicate_operation_registrations: u64,
    pub serve_start_count: u64,
    pub product_sessions_created: u64,
    pub product_session_preview_creations: u64,
    pub product_session_mutation_creations: u64,
    pub product_session_lookups_attempted: u64,
    pub product_session_lookups_denied_missing: u64,
    pub product_session_lookups_denied_foreign: u64,
    pub product_session_lookups_denied_expired: u64,
    pub product_session_lookups_denied_closed: u64,
    pub product_session_lookups_denied_moved: u64,
    pub product_session_lookups_denied_preview_for_mutation: u64,
    pub product_session_closes_recorded: u64,
    pub product_result_artifacts_emitted: u64,
    pub product_result_bytes_emitted: u64,
    pub product_result_oversized_denials: u64,
    pub durable_product_mutation_attempts: u64,
    pub durable_product_basis_comparisons: u64,
    pub durable_product_commits: u64,
    pub durable_product_previously_committed: u64,
    pub durable_product_idempotency_conflicts: u64,
    pub durable_product_stale_bases: u64,
    pub durable_product_indeterminate: u64,
    pub durable_product_recovery_attempts: u64,
    pub durable_product_recovery_resolved: u64,
    pub durable_product_recovery_failed: u64,
}
