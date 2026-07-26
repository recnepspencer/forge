use super::{
    speculative_index, PhysicalResidencyAllocationEventRecorder, PhysicalResidencyCounters,
};
use crate::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalSpeculativeWorkKind,
};

#[path = "resource_accounting/lifecycle.rs"]
mod lifecycle;
#[path = "resource_accounting/speculative.rs"]
mod speculative;

#[derive(Debug)]
pub(crate) struct PhysicalResidencyAccounting {
    counters: PhysicalResidencyCounters,
    events: PhysicalResidencyAllocationEventRecorder,
}

impl PhysicalResidencyAccounting {
    pub(crate) fn new(
        metadata_bytes: u64,
        events: PhysicalResidencyAllocationEventRecorder,
    ) -> Self {
        events.admit(PhysicalResidencyDimension::MetadataBytes, metadata_bytes);
        events.admit(PhysicalResidencyDimension::TotalBytes, metadata_bytes);
        Self {
            counters: PhysicalResidencyCounters {
                metadata_bytes,
                peak_metadata_bytes: metadata_bytes,
                peak_admitted_bytes: metadata_bytes,
                ..PhysicalResidencyCounters::default()
            },
            events,
        }
    }

    pub(crate) const fn snapshot(&self) -> PhysicalResidencyCounters {
        self.counters
    }

    pub(crate) const fn resident_bytes(&self) -> u64 {
        self.counters.resident_bytes
    }

    pub(crate) const fn frame_entries(&self) -> u32 {
        self.counters.frame_entries
    }

    pub(crate) const fn pinned_frames(&self) -> u32 {
        self.counters.pinned_frames
    }

    pub(crate) const fn pin_leases(&self) -> u32 {
        self.counters.pin_leases
    }

    pub(crate) const fn dirty_frames(&self) -> u32 {
        self.counters.dirty_frames
    }

    pub(crate) const fn dirty_replacement_bytes(&self) -> u64 {
        self.counters.dirty_replacement_bytes
    }

    pub(crate) const fn active_operation_bytes(&self) -> u64 {
        self.counters.active_operation_bytes
    }

    pub(crate) const fn operation_scope_bytes(
        &self,
        scope: PhysicalOperationAllocationScope,
    ) -> u64 {
        self.counters.operation_scope_bytes[scope.index()]
    }

    pub(crate) const fn active_speculative_frames(&self, kind: PhysicalSpeculativeWorkKind) -> u32 {
        self.counters.active_speculative_frames[speculative_index(kind)]
    }

    pub(crate) const fn admitted_bytes(&self) -> u64 {
        self.counters.admitted_bytes()
    }

    pub(crate) fn deny(&mut self) {
        self.counters.denials += 1;
    }

    pub(crate) fn deny_dimension(&mut self, dimension: PhysicalResidencyDimension, requested: u64) {
        self.deny();
        self.events.deny(dimension, requested);
    }

    pub(crate) fn admit_operation(&mut self, scope: PhysicalOperationAllocationScope, bytes: u64) {
        self.counters.active_operation_bytes += bytes;
        self.counters.peak_operation_bytes = self
            .counters
            .peak_operation_bytes
            .max(self.counters.active_operation_bytes);
        self.counters.operation_scope_bytes[scope.index()] += bytes;
        self.counters.peak_operation_scope_bytes[scope.index()] =
            self.counters.peak_operation_scope_bytes[scope.index()]
                .max(self.counters.operation_scope_bytes[scope.index()]);
        self.events
            .admit(PhysicalResidencyDimension::OperationBytes, bytes);
        self.events
            .admit(PhysicalResidencyDimension::OperationScope(scope), bytes);
        self.events
            .admit(PhysicalResidencyDimension::TotalBytes, bytes);
        self.observe_admitted_peak();
    }

    pub(crate) fn release_operation(
        &mut self,
        scope: PhysicalOperationAllocationScope,
        bytes: u64,
    ) {
        self.counters.active_operation_bytes -= bytes;
        self.counters.operation_scope_bytes[scope.index()] -= bytes;
        self.events
            .release(PhysicalResidencyDimension::OperationBytes, bytes);
        self.events
            .release(PhysicalResidencyDimension::OperationScope(scope), bytes);
        self.events
            .release(PhysicalResidencyDimension::TotalBytes, bytes);
    }

    pub(crate) fn admit_frame(&mut self, bytes: u64, dirty: bool, candidate: bool) {
        self.counters.resident_bytes += bytes;
        self.counters.frame_entries += 1;
        self.counters.pinned_frames += 1;
        self.counters.pin_leases += 1;
        self.counters.active_loading_frames += 1;
        if dirty {
            self.counters.dirty_frames += 1;
        } else {
            self.counters.faults += 1;
        }
        if candidate {
            self.counters.candidate_frames += 1;
        }
        self.events
            .admit(PhysicalResidencyDimension::ResidentBytes, bytes);
        self.events
            .admit(PhysicalResidencyDimension::TotalBytes, bytes);
        self.events
            .admit(PhysicalResidencyDimension::FrameEntries, 1);
        self.events
            .admit(PhysicalResidencyDimension::PinnedFrames, 1);
        self.events.admit(PhysicalResidencyDimension::PinLeases, 1);
        if dirty {
            self.events
                .admit(PhysicalResidencyDimension::DirtyFrames, 1);
        }
        self.observe_frame_peaks();
        self.observe_admitted_peak();
    }

    pub(crate) fn finish_loading(&mut self) {
        self.counters.active_loading_frames -= 1;
    }

    pub(crate) fn resolve_bounded_frame(&mut self, reserved: u64, actual: u64) {
        let released = reserved
            .checked_sub(actual)
            .expect("a bounded frame cannot exceed its admitted reservation");
        self.counters.resident_bytes -= released;
        self.events
            .release(PhysicalResidencyDimension::ResidentBytes, released);
        self.events
            .release(PhysicalResidencyDimension::TotalBytes, released);
    }

    pub(crate) fn attach_loading_waiter(&mut self) {
        self.counters.pin_leases += 1;
        self.counters.coalesced_waiters += 1;
        self.counters.peak_pin_leases = self.counters.peak_pin_leases.max(self.counters.pin_leases);
        self.events.admit(PhysicalResidencyDimension::PinLeases, 1);
    }

    pub(crate) fn fail_loading(&mut self, bytes: u64, pins: u32, retain_identity: bool) {
        self.counters.resident_bytes -= bytes;
        self.counters.pin_leases -= pins;
        self.counters.pinned_frames -= 1;
        self.counters.active_loading_frames -= 1;
        self.events
            .release(PhysicalResidencyDimension::ResidentBytes, bytes);
        self.events
            .release(PhysicalResidencyDimension::TotalBytes, bytes);
        self.events
            .release(PhysicalResidencyDimension::PinLeases, u64::from(pins));
        self.events
            .release(PhysicalResidencyDimension::PinnedFrames, 1);
        if !retain_identity {
            self.release_failed_loading_identity();
        }
    }

    pub(crate) fn release_failed_loading_identity(&mut self) {
        self.counters.frame_entries -= 1;
        self.events
            .release(PhysicalResidencyDimension::FrameEntries, 1);
    }

    pub(crate) fn remove_frame(&mut self, bytes: u64, pins: u32, dirty: bool, candidate: bool) {
        self.counters.resident_bytes -= bytes;
        self.counters.frame_entries -= 1;
        self.counters.pin_leases -= pins;
        if pins > 0 {
            self.counters.pinned_frames -= 1;
        }
        if dirty {
            self.counters.dirty_frames -= 1;
        }
        if candidate {
            self.counters.candidate_frames -= 1;
        }
        self.events
            .release(PhysicalResidencyDimension::ResidentBytes, bytes);
        self.events
            .release(PhysicalResidencyDimension::TotalBytes, bytes);
        self.events
            .release(PhysicalResidencyDimension::FrameEntries, 1);
        self.events
            .release(PhysicalResidencyDimension::PinLeases, u64::from(pins));
        if pins > 0 {
            self.events
                .release(PhysicalResidencyDimension::PinnedFrames, 1);
        }
        if dirty {
            self.events
                .release(PhysicalResidencyDimension::DirtyFrames, 1);
        }
    }

    pub(crate) fn pin(&mut self, newly_pinned: bool) {
        self.counters.pin_leases += 1;
        self.events.admit(PhysicalResidencyDimension::PinLeases, 1);
        if newly_pinned {
            self.counters.pinned_frames += 1;
            self.events
                .admit(PhysicalResidencyDimension::PinnedFrames, 1);
        }
        self.counters.peak_pinned_frames = self
            .counters
            .peak_pinned_frames
            .max(self.counters.pinned_frames);
        self.counters.peak_pin_leases = self.counters.peak_pin_leases.max(self.counters.pin_leases);
        self.counters.hits += 1;
    }

    pub(crate) fn unpin(&mut self, became_unpinned: bool) {
        self.counters.pin_leases -= 1;
        self.events
            .release(PhysicalResidencyDimension::PinLeases, 1);
        if became_unpinned {
            self.counters.pinned_frames -= 1;
            self.events
                .release(PhysicalResidencyDimension::PinnedFrames, 1);
        }
    }

    pub(crate) fn mark_dirty(&mut self, candidate_added: bool) {
        self.counters.dirty_frames += 1;
        self.counters.dirty_transitions += 1;
        self.events
            .admit(PhysicalResidencyDimension::DirtyFrames, 1);
        if candidate_added {
            self.counters.candidate_frames += 1;
        }
        self.counters.peak_dirty_frames = self
            .counters
            .peak_dirty_frames
            .max(self.counters.dirty_frames);
        self.counters.peak_candidate_frames = self
            .counters
            .peak_candidate_frames
            .max(self.counters.candidate_frames);
    }

    pub(crate) fn mark_clean(&mut self, candidate_removed: bool, writeback: bool) {
        self.counters.dirty_frames -= 1;
        self.events
            .release(PhysicalResidencyDimension::DirtyFrames, 1);
        if candidate_removed {
            self.counters.candidate_frames -= 1;
            self.counters.candidate_publications += 1;
        }
        if writeback {
            self.counters.writebacks += 1;
        }
    }

    pub(crate) fn reserve_dirty_replacement(&mut self, bytes: u64) {
        self.counters.dirty_replacement_bytes += bytes;
        self.counters.peak_dirty_replacement_bytes = self
            .counters
            .peak_dirty_replacement_bytes
            .max(self.counters.dirty_replacement_bytes);
        self.events
            .admit(PhysicalResidencyDimension::DirtyReplacementBytes, bytes);
        self.events
            .admit(PhysicalResidencyDimension::TotalBytes, bytes);
        self.observe_admitted_peak();
    }

    pub(crate) fn release_dirty_replacement(&mut self, bytes: u64) {
        self.counters.dirty_replacement_bytes -= bytes;
        self.events
            .release(PhysicalResidencyDimension::DirtyReplacementBytes, bytes);
        self.events
            .release(PhysicalResidencyDimension::TotalBytes, bytes);
    }

    pub(crate) fn dirty_replacement_allocator_failed(&mut self, bytes: u64) {
        self.counters.dirty_replacement_bytes -= bytes;
        self.counters.denials += 1;
        self.events
            .allocator_failure(PhysicalResidencyDimension::DirtyReplacementBytes, bytes);
        self.events
            .allocator_failure(PhysicalResidencyDimension::TotalBytes, bytes);
    }

    pub(crate) fn record_source_load(&mut self) {
        self.counters.source_loads += 1;
    }

    pub(crate) fn record_copy(&mut self, bytes: u64) {
        self.counters.copy_operations += 1;
        self.counters.copied_bytes = self.counters.copied_bytes.saturating_add(bytes);
        self.counters.maximum_copy_width = self.counters.maximum_copy_width.max(bytes);
    }

    pub(crate) fn inspect_eviction_candidate(&mut self) {
        self.counters.eviction_candidate_inspections += 1;
    }

    pub(crate) fn record_eviction(&mut self) {
        self.counters.evictions += 1;
    }

    pub(crate) fn record_administrative_drain(&mut self) {
        self.counters.administrative_drains += 1;
    }

    pub(crate) fn record_identity_transition(&mut self) {
        self.counters.identity_transitions += 1;
    }

    fn observe_frame_peaks(&mut self) {
        self.counters.peak_resident_bytes = self
            .counters
            .peak_resident_bytes
            .max(self.counters.resident_bytes);
        self.counters.peak_frame_entries = self
            .counters
            .peak_frame_entries
            .max(self.counters.frame_entries);
        self.counters.peak_pinned_frames = self
            .counters
            .peak_pinned_frames
            .max(self.counters.pinned_frames);
        self.counters.peak_pin_leases = self.counters.peak_pin_leases.max(self.counters.pin_leases);
        self.counters.peak_dirty_frames = self
            .counters
            .peak_dirty_frames
            .max(self.counters.dirty_frames);
        self.counters.peak_candidate_frames = self
            .counters
            .peak_candidate_frames
            .max(self.counters.candidate_frames);
    }

    fn observe_admitted_peak(&mut self) {
        self.counters.peak_admitted_bytes = self
            .counters
            .peak_admitted_bytes
            .max(self.counters.admitted_bytes());
    }
}
