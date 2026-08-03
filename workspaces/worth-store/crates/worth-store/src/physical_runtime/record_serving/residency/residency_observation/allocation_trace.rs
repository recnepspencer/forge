/// Ordered allocation-boundary evidence retained only for certification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationTrace {
    inner: worth_store_buffer_pool::PhysicalResidencyAllocationTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationBoundaryEvent {
    inner: worth_store_buffer_pool::PhysicalResidencyAllocationBoundaryEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResidencyAllocationBoundaryKind {
    Admission,
    Release,
    Denial,
    AllocatorFailure,
    Actualization,
}

impl PhysicalResidencyAllocationTrace {
    pub(in crate::physical_runtime::record_serving) const fn new(
        inner: worth_store_buffer_pool::PhysicalResidencyAllocationTrace,
    ) -> Self {
        Self { inner }
    }

    pub const fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.inner.store()
    }

    pub const fn pool_incarnation(&self) -> u64 {
        self.inner.pool().get()
    }

    pub fn event_count(&self) -> usize {
        self.inner.events().len()
    }

    pub fn events(
        &self,
    ) -> impl ExactSizeIterator<Item = PhysicalResidencyAllocationBoundaryEvent> + '_ {
        self.inner
            .events()
            .iter()
            .copied()
            .map(|inner| PhysicalResidencyAllocationBoundaryEvent { inner })
    }
}

impl PhysicalResidencyAllocationBoundaryEvent {
    pub const fn sequence(self) -> u64 {
        self.inner.sequence()
    }

    pub const fn kind(self) -> PhysicalResidencyAllocationBoundaryKind {
        PhysicalResidencyAllocationBoundaryKind::from_lower(self.inner.kind())
    }

    pub const fn dimension(self) -> worth_store_buffer_pool::PhysicalResidencyDimension {
        self.inner.dimension()
    }

    pub const fn scope(self) -> Option<worth_store_buffer_pool::PhysicalOperationAllocationScope> {
        self.inner.scope()
    }

    pub const fn requested_units(self) -> u64 {
        self.inner.requested_units()
    }

    pub const fn actual_units(self) -> u64 {
        self.inner.actual_units()
    }

    pub const fn process(self) -> u32 {
        self.inner.process()
    }

    pub const fn physical_operation(self) -> Option<u64> {
        match self.inner.operation() {
            Some(operation) => Some(operation.get()),
            None => None,
        }
    }
}

impl PhysicalResidencyAllocationBoundaryKind {
    const fn from_lower(
        kind: worth_store_buffer_pool::PhysicalResidencyAllocationBoundaryKind,
    ) -> Self {
        use worth_store_buffer_pool::PhysicalResidencyAllocationBoundaryKind as Lower;
        match kind {
            Lower::Admission => Self::Admission,
            Lower::Release => Self::Release,
            Lower::Denial => Self::Denial,
            Lower::AllocatorFailure => Self::AllocatorFailure,
            Lower::Actualization => Self::Actualization,
        }
    }
}
