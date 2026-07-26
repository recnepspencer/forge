use std::sync::{Arc, Mutex};

use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::{PhysicalResidencyDimension, PhysicalResidencyIncarnation};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationEventCounters {
    attempts: u64,
    admissions: u64,
    releases: u64,
    denials: u64,
    allocator_failures: u64,
    admitted_units: u64,
    released_units: u64,
    denied_units: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyAllocationEventSnapshot {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    dimensions: [PhysicalResidencyAllocationEventCounters; PhysicalResidencyDimension::COUNT],
}

#[derive(Debug, Clone)]
pub struct PhysicalResidencyAllocationEventObserver {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    cells: Arc<Mutex<PhysicalResidencyAllocationEventCells>>,
}

#[derive(Debug, Clone)]
pub(crate) struct PhysicalResidencyAllocationEventRecorder {
    cells: Arc<Mutex<PhysicalResidencyAllocationEventCells>>,
}

#[derive(Debug)]
struct PhysicalResidencyAllocationEventCells {
    dimensions: [PhysicalResidencyAllocationEventCounters; PhysicalResidencyDimension::COUNT],
}

impl PhysicalResidencyAllocationEventRecorder {
    pub(crate) fn new(
        store: StableStoreIdentity,
        pool: PhysicalResidencyIncarnation,
    ) -> (Self, PhysicalResidencyAllocationEventObserver) {
        let cells = Arc::new(Mutex::new(PhysicalResidencyAllocationEventCells {
            dimensions: [PhysicalResidencyAllocationEventCounters::default();
                PhysicalResidencyDimension::COUNT],
        }));
        (
            Self {
                cells: Arc::clone(&cells),
            },
            PhysicalResidencyAllocationEventObserver { store, pool, cells },
        )
    }

    pub(crate) fn admit(&self, dimension: PhysicalResidencyDimension, units: u64) {
        let mut cells = self.lock();
        let counters = &mut cells.dimensions[dimension.index()];
        counters.attempts += 1;
        counters.admissions += 1;
        counters.admitted_units = counters.admitted_units.saturating_add(units);
    }

    pub(crate) fn release(&self, dimension: PhysicalResidencyDimension, units: u64) {
        let mut cells = self.lock();
        let counters = &mut cells.dimensions[dimension.index()];
        counters.releases += 1;
        counters.released_units = counters.released_units.saturating_add(units);
    }

    pub(crate) fn deny(&self, dimension: PhysicalResidencyDimension, units: u64) {
        let mut cells = self.lock();
        let counters = &mut cells.dimensions[dimension.index()];
        counters.attempts += 1;
        counters.denials += 1;
        counters.denied_units = counters.denied_units.saturating_add(units);
    }

    pub(crate) fn allocator_failure(&self, dimension: PhysicalResidencyDimension, units: u64) {
        let mut cells = self.lock();
        let counters = &mut cells.dimensions[dimension.index()];
        counters.allocator_failures += 1;
        counters.releases += 1;
        counters.released_units = counters.released_units.saturating_add(units);
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, PhysicalResidencyAllocationEventCells> {
        self.cells
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl PhysicalResidencyAllocationEventObserver {
    pub fn snapshot(&self) -> PhysicalResidencyAllocationEventSnapshot {
        let cells = self
            .cells
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        PhysicalResidencyAllocationEventSnapshot {
            store: self.store,
            pool: self.pool,
            dimensions: cells.dimensions,
        }
    }
}

impl PhysicalResidencyAllocationEventSnapshot {
    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool(self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn for_dimension(
        self,
        dimension: PhysicalResidencyDimension,
    ) -> PhysicalResidencyAllocationEventCounters {
        self.dimensions[dimension.index()]
    }
}

impl PhysicalResidencyAllocationEventCounters {
    pub const fn attempts(self) -> u64 {
        self.attempts
    }

    pub const fn admissions(self) -> u64 {
        self.admissions
    }

    pub const fn releases(self) -> u64 {
        self.releases
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }

    pub const fn allocator_failures(self) -> u64 {
        self.allocator_failures
    }

    pub const fn admitted_units(self) -> u64 {
        self.admitted_units
    }

    pub const fn released_units(self) -> u64 {
        self.released_units
    }

    pub const fn denied_units(self) -> u64 {
        self.denied_units
    }

    pub const fn active_units(self) -> u64 {
        self.admitted_units.saturating_sub(self.released_units)
    }
}
