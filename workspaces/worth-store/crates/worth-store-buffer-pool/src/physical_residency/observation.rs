#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhysicalResidencyCounters {
    pub(crate) metadata_bytes: u64,
    pub(crate) resident_bytes: u64,
    pub(crate) peak_resident_bytes: u64,
    pub(crate) pinned_frames: u32,
    pub(crate) peak_pinned_frames: u32,
    pub(crate) pin_leases: u32,
    pub(crate) peak_pin_leases: u32,
    pub(crate) dirty_frames: u32,
    pub(crate) peak_dirty_frames: u32,
    pub(crate) candidate_frames: u32,
    pub(crate) peak_candidate_frames: u32,
    pub(crate) active_loading_frames: u32,
    pub(crate) active_operation_bytes: u64,
    pub(crate) peak_operation_bytes: u64,
    pub(crate) peak_admitted_bytes: u64,
    pub(crate) hits: u64,
    pub(crate) faults: u64,
    pub(crate) source_loads: u64,
    pub(crate) evictions: u64,
    pub(crate) eviction_candidate_inspections: u64,
    pub(crate) administrative_drains: u64,
    pub(crate) writebacks: u64,
    pub(crate) candidate_publications: u64,
    pub(crate) copied_bytes: u64,
    pub(crate) copy_operations: u64,
    pub(crate) denials: u64,
    pub(crate) identity_transitions: u64,
    pub(crate) speculative_attempts: [u64; 3],
    pub(crate) speculative_admissions: [u64; 3],
    pub(crate) active_speculative_frames: [u32; 3],
    pub(crate) peak_speculative_frames: [u32; 3],
    pub(crate) operation_scope_bytes: [u64; 7],
    pub(crate) peak_operation_scope_bytes: [u64; 7],
}

impl PhysicalResidencyCounters {
    pub const fn metadata_bytes(self) -> u64 {
        self.metadata_bytes
    }
    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }
    pub const fn peak_resident_bytes(self) -> u64 {
        self.peak_resident_bytes
    }
    pub const fn pinned_frames(self) -> u32 {
        self.pinned_frames
    }
    pub const fn peak_pinned_frames(self) -> u32 {
        self.peak_pinned_frames
    }
    pub const fn pin_leases(self) -> u32 {
        self.pin_leases
    }
    pub const fn peak_pin_leases(self) -> u32 {
        self.peak_pin_leases
    }
    pub const fn dirty_frames(self) -> u32 {
        self.dirty_frames
    }
    pub const fn peak_dirty_frames(self) -> u32 {
        self.peak_dirty_frames
    }
    pub const fn candidate_frames(self) -> u32 {
        self.candidate_frames
    }
    pub const fn peak_candidate_frames(self) -> u32 {
        self.peak_candidate_frames
    }
    pub const fn active_loading_frames(self) -> u32 {
        self.active_loading_frames
    }
    pub const fn active_operation_bytes(self) -> u64 {
        self.active_operation_bytes
    }
    pub const fn peak_operation_bytes(self) -> u64 {
        self.peak_operation_bytes
    }
    pub const fn admitted_bytes(self) -> u64 {
        self.metadata_bytes
            .saturating_add(self.resident_bytes)
            .saturating_add(self.active_operation_bytes)
    }
    pub const fn peak_admitted_bytes(self) -> u64 {
        self.peak_admitted_bytes
    }
    pub const fn hits(self) -> u64 {
        self.hits
    }
    pub const fn faults(self) -> u64 {
        self.faults
    }
    pub const fn source_loads(self) -> u64 {
        self.source_loads
    }
    pub const fn evictions(self) -> u64 {
        self.evictions
    }
    pub const fn eviction_candidate_inspections(self) -> u64 {
        self.eviction_candidate_inspections
    }
    pub const fn administrative_drains(self) -> u64 {
        self.administrative_drains
    }
    pub const fn writebacks(self) -> u64 {
        self.writebacks
    }
    pub const fn candidate_publications(self) -> u64 {
        self.candidate_publications
    }
    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }
    pub const fn copy_operations(self) -> u64 {
        self.copy_operations
    }
    pub const fn denials(self) -> u64 {
        self.denials
    }
    pub const fn identity_transitions(self) -> u64 {
        self.identity_transitions
    }
    pub const fn speculative_attempts(self, kind: crate::SpeculativePhysicalWorkKind) -> u64 {
        self.speculative_attempts[speculative_index(kind)]
    }
    pub const fn speculative_admissions(self, kind: crate::SpeculativePhysicalWorkKind) -> u64 {
        self.speculative_admissions[speculative_index(kind)]
    }
    pub const fn active_speculative_frames(self, kind: crate::SpeculativePhysicalWorkKind) -> u32 {
        self.active_speculative_frames[speculative_index(kind)]
    }
    pub const fn peak_speculative_frames(self, kind: crate::SpeculativePhysicalWorkKind) -> u32 {
        self.peak_speculative_frames[speculative_index(kind)]
    }
    pub const fn active_operation_bytes_for(self, scope: super::OperationAllocationScope) -> u64 {
        self.operation_scope_bytes[scope.index()]
    }
    pub const fn peak_operation_bytes_for(self, scope: super::OperationAllocationScope) -> u64 {
        self.peak_operation_scope_bytes[scope.index()]
    }
}

pub(crate) const fn speculative_index(kind: crate::SpeculativePhysicalWorkKind) -> usize {
    match kind {
        crate::SpeculativePhysicalWorkKind::ReadAhead => 0,
        crate::SpeculativePhysicalWorkKind::Prefetch => 1,
        crate::SpeculativePhysicalWorkKind::WriteBehind => 2,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyShutdown {
    counters: PhysicalResidencyCounters,
}

impl PhysicalResidencyShutdown {
    pub(crate) const fn new(counters: PhysicalResidencyCounters) -> Self {
        Self { counters }
    }
    pub const fn counters(self) -> PhysicalResidencyCounters {
        self.counters
    }
    pub const fn requires_inspection(self) -> bool {
        self.counters.pin_leases > self.counters.active_loading_frames
            || self.counters.dirty_frames > 0
            || self.counters.active_speculative_frames[2] != 0
    }

    pub const fn cancelled_read_work(self) -> bool {
        self.counters.active_operation_bytes > 0
            || self.counters.active_loading_frames > 0
            || self.counters.active_speculative_frames[0] != 0
            || self.counters.active_speculative_frames[1] != 0
    }
}
