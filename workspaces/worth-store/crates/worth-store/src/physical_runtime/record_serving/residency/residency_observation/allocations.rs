#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationSnapshot {
    inner: worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationEventSnapshot {
    inner: worth_store_buffer_pool::PhysicalResidencyAllocationEventCounters,
}

impl PhysicalResidencyAllocationSnapshot {
    pub(super) const fn new(
        inner: worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot,
    ) -> Self {
        Self { inner }
    }

    pub const fn store_identity(
        self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.inner.store()
    }

    pub const fn for_dimension(
        self,
        dimension: crate::physical_runtime::record_serving::PhysicalResidencyDimension,
    ) -> PhysicalResidencyAllocationEventSnapshot {
        PhysicalResidencyAllocationEventSnapshot {
            inner: self.inner.for_dimension(dimension),
        }
    }
}

impl PhysicalResidencyAllocationEventSnapshot {
    pub const fn attempts(self) -> u64 {
        self.inner.attempts()
    }
    pub const fn admissions(self) -> u64 {
        self.inner.admissions()
    }
    pub const fn releases(self) -> u64 {
        self.inner.releases()
    }
    pub const fn denials(self) -> u64 {
        self.inner.denials()
    }
    pub const fn allocator_failures(self) -> u64 {
        self.inner.allocator_failures()
    }
    pub const fn admitted_units(self) -> u64 {
        self.inner.admitted_units()
    }
    pub const fn released_units(self) -> u64 {
        self.inner.released_units()
    }
    pub const fn denied_units(self) -> u64 {
        self.inner.denied_units()
    }
    pub const fn active_units(self) -> u64 {
        self.inner.active_units()
    }
}
