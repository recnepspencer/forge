/// Current, peak, and transition counters for one Store's physical residency.
///
/// Counters describe executed transitions. They cannot grant a lease, prove an
/// effect, or authorize retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyCounterSnapshot {
    inner: worth_store_buffer_pool::PhysicalResidencyCounters,
}

impl PhysicalResidencyCounterSnapshot {
    pub(super) const fn new(inner: worth_store_buffer_pool::PhysicalResidencyCounters) -> Self {
        Self { inner }
    }

    pub const fn metadata_bytes(self) -> u64 {
        self.inner.metadata_bytes()
    }
    pub const fn peak_metadata_bytes(self) -> u64 {
        self.inner.peak_metadata_bytes()
    }
    pub const fn resident_bytes(self) -> u64 {
        self.inner.resident_bytes()
    }
    pub const fn peak_resident_bytes(self) -> u64 {
        self.inner.peak_resident_bytes()
    }
    pub const fn dirty_replacement_bytes(self) -> u64 {
        self.inner.dirty_replacement_bytes()
    }
    pub const fn peak_dirty_replacement_bytes(self) -> u64 {
        self.inner.peak_dirty_replacement_bytes()
    }
    pub const fn frame_entries(self) -> u32 {
        self.inner.frame_entries()
    }
    pub const fn peak_frame_entries(self) -> u32 {
        self.inner.peak_frame_entries()
    }
    pub const fn pinned_frames(self) -> u32 {
        self.inner.pinned_frames()
    }
    pub const fn peak_pinned_frames(self) -> u32 {
        self.inner.peak_pinned_frames()
    }
    pub const fn pin_leases(self) -> u32 {
        self.inner.pin_leases()
    }
    pub const fn peak_pin_leases(self) -> u32 {
        self.inner.peak_pin_leases()
    }
    pub const fn dirty_frames(self) -> u32 {
        self.inner.dirty_frames()
    }
    pub const fn peak_dirty_frames(self) -> u32 {
        self.inner.peak_dirty_frames()
    }
    pub const fn candidate_frames(self) -> u32 {
        self.inner.candidate_frames()
    }
    pub const fn peak_candidate_frames(self) -> u32 {
        self.inner.peak_candidate_frames()
    }
    pub const fn active_loading_frames(self) -> u32 {
        self.inner.active_loading_frames()
    }
    pub const fn active_writeback_claims(self) -> u32 {
        self.inner.active_writeback_claims()
    }
    pub const fn peak_writeback_claims(self) -> u32 {
        self.inner.peak_writeback_claims()
    }
    pub const fn active_operation_bytes(self) -> u64 {
        self.inner.active_operation_bytes()
    }
    pub const fn peak_operation_bytes(self) -> u64 {
        self.inner.peak_operation_bytes()
    }
    pub const fn admitted_bytes(self) -> u64 {
        self.inner.admitted_bytes()
    }
    pub const fn peak_admitted_bytes(self) -> u64 {
        self.inner.peak_admitted_bytes()
    }
    pub const fn hits(self) -> u64 {
        self.inner.hits()
    }
    pub const fn faults(self) -> u64 {
        self.inner.faults()
    }
    pub const fn source_loads(self) -> u64 {
        self.inner.source_loads()
    }
    pub const fn coalesced_waiters(self) -> u64 {
        self.inner.coalesced_waiters()
    }
    pub const fn evictions(self) -> u64 {
        self.inner.evictions()
    }
    pub const fn eviction_candidate_inspections(self) -> u64 {
        self.inner.eviction_candidate_inspections()
    }
    pub const fn administrative_drains(self) -> u64 {
        self.inner.administrative_drains()
    }
    pub const fn writebacks(self) -> u64 {
        self.inner.writebacks()
    }
    pub const fn candidate_publications(self) -> u64 {
        self.inner.candidate_publications()
    }
    pub const fn copied_bytes(self) -> u64 {
        self.inner.copied_bytes()
    }
    pub const fn copy_operations(self) -> u64 {
        self.inner.copy_operations()
    }
    pub const fn maximum_copy_width(self) -> u64 {
        self.inner.maximum_copy_width()
    }
    pub const fn dirty_transitions(self) -> u64 {
        self.inner.dirty_transitions()
    }
    pub const fn denials(self) -> u64 {
        self.inner.denials()
    }
    pub const fn identity_transitions(self) -> u64 {
        self.inner.identity_transitions()
    }
    pub const fn speculative_attempts(
        self,
        kind: crate::physical_runtime::record_serving::PhysicalSpeculativeWorkKind,
    ) -> u64 {
        self.inner.speculative_attempts(kind)
    }
    pub const fn speculative_admissions(
        self,
        kind: crate::physical_runtime::record_serving::PhysicalSpeculativeWorkKind,
    ) -> u64 {
        self.inner.speculative_admissions(kind)
    }
    pub const fn speculative_completions(
        self,
        kind: crate::physical_runtime::record_serving::PhysicalSpeculativeWorkKind,
    ) -> u64 {
        self.inner.speculative_completions(kind)
    }
    pub const fn speculative_denials(
        self,
        kind: crate::physical_runtime::record_serving::PhysicalSpeculativeWorkKind,
    ) -> u64 {
        self.inner.speculative_denials(kind)
    }
    pub const fn active_speculative_frames(
        self,
        kind: crate::physical_runtime::record_serving::PhysicalSpeculativeWorkKind,
    ) -> u32 {
        self.inner.active_speculative_frames(kind)
    }
    pub const fn peak_speculative_frames(
        self,
        kind: crate::physical_runtime::record_serving::PhysicalSpeculativeWorkKind,
    ) -> u32 {
        self.inner.peak_speculative_frames(kind)
    }
    pub const fn active_operation_bytes_for(
        self,
        scope: crate::physical_runtime::record_serving::PhysicalOperationAllocationScope,
    ) -> u64 {
        self.inner.active_operation_bytes_for(scope)
    }
    pub const fn peak_operation_bytes_for(
        self,
        scope: crate::physical_runtime::record_serving::PhysicalOperationAllocationScope,
    ) -> u64 {
        self.inner.peak_operation_bytes_for(scope)
    }
}
