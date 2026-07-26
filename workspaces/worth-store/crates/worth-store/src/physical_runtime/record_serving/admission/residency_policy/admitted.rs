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

    pub const fn total_bytes(self) -> u64 {
        self.limits.total_bytes()
    }

    pub const fn resident_bytes(self) -> u64 {
        self.limits.resident_bytes()
    }

    pub const fn metadata_bytes(self) -> u64 {
        self.limits.metadata_bytes()
    }

    pub const fn frame_entries(self) -> u32 {
        self.limits.frame_entries()
    }

    pub const fn pinned_frames(self) -> u32 {
        self.limits.pinned_frames()
    }

    pub const fn pin_leases(self) -> u32 {
        self.limits.pin_leases()
    }

    pub const fn dirty_frames(self) -> u32 {
        self.limits.dirty_frames()
    }

    pub const fn dirty_replacement_bytes(self) -> u64 {
        self.limits.dirty_replacement_bytes()
    }

    pub const fn operation_bytes(self) -> u64 {
        self.limits.operation_bytes()
    }

    pub const fn scope_bytes(self, scope: super::PhysicalOperationAllocationScope) -> u64 {
        self.limits.scope_bytes(scope)
    }

    pub const fn speculative_frames(self, kind: super::PhysicalSpeculativeWorkKind) -> u32 {
        self.limits.speculative_frames(kind)
    }
}
