/// A complete residency policy admitted against one physical record format.
///
/// Store retains this sealed value when it constructs the instance's single
/// buffer pool. Its getters are configuration evidence, not allocation,
/// eviction, or retry authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPhysicalRecordResidencyPolicy {
    pub(super) limits: worth_store_buffer_pool::PhysicalResidencyLimits,
}

impl AdmittedPhysicalRecordResidencyPolicy {
    pub(in crate::physical_runtime) const fn limits(
        self,
    ) -> worth_store_buffer_pool::PhysicalResidencyLimits {
        self.limits
    }

    /// Returns the hard envelope for all live pool-owned bytes.
    pub const fn total_bytes(self) -> u64 {
        self.limits.total_bytes()
    }

    /// Returns the resident frame-payload byte ceiling.
    pub const fn resident_bytes(self) -> u64 {
        self.limits.resident_bytes()
    }

    /// Returns the frame-table metadata byte ceiling.
    pub const fn metadata_bytes(self) -> u64 {
        self.limits.metadata_bytes()
    }

    /// Returns the frame identity ceiling.
    pub const fn frame_entries(self) -> u32 {
        self.limits.frame_entries()
    }

    /// Returns the simultaneously pinned frame ceiling.
    pub const fn pinned_frames(self) -> u32 {
        self.limits.pinned_frames()
    }

    /// Returns the live pin-lease ceiling.
    pub const fn pin_leases(self) -> u32 {
        self.limits.pin_leases()
    }

    /// Returns the dirty frame ceiling.
    pub const fn dirty_frames(self) -> u32 {
        self.limits.dirty_frames()
    }

    /// Returns the dirty replacement byte ceiling.
    pub const fn dirty_replacement_bytes(self) -> u64 {
        self.limits.dirty_replacement_bytes()
    }

    /// Returns the aggregate operation-owned byte ceiling.
    pub const fn operation_bytes(self) -> u64 {
        self.limits.operation_bytes()
    }

    /// Returns one operation scope's byte ceiling.
    pub const fn scope_bytes(self, scope: super::PhysicalOperationAllocationScope) -> u64 {
        self.limits.scope_bytes(scope)
    }

    /// Returns one speculative work kind's frame ceiling.
    pub const fn speculative_frames(self, kind: super::PhysicalSpeculativeWorkKind) -> u32 {
        self.limits.speculative_frames(kind)
    }
}
