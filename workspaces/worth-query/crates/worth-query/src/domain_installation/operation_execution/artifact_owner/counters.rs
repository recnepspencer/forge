#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryArtifactLifecycleCounters {
    pub production_admissions: usize,
    pub owner_registrations: usize,
    pub transfer_admissions: usize,
    pub borrow_admissions: usize,
    pub lease_admissions: usize,
    pub lifecycle_generation_checks: usize,
    pub provider_disposals: usize,
    pub retained_bytes: usize,
    pub peak_retained_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactOwnerSnapshot {
    owner_count: usize,
    borrow_count: usize,
    lease_count: usize,
    lifecycle_generation: u64,
    disposed: bool,
    counters: WorthQueryArtifactLifecycleCounters,
}

impl WorthQueryArtifactOwnerSnapshot {
    pub const fn owner_count(self) -> usize {
        self.owner_count
    }

    pub const fn borrow_count(self) -> usize {
        self.borrow_count
    }

    pub const fn lease_count(self) -> usize {
        self.lease_count
    }

    pub const fn lifecycle_generation(self) -> u64 {
        self.lifecycle_generation
    }

    pub const fn is_disposed(self) -> bool {
        self.disposed
    }

    pub const fn counters(self) -> WorthQueryArtifactLifecycleCounters {
        self.counters
    }

    pub(super) const fn new(
        owner_count: usize,
        borrow_count: usize,
        lease_count: usize,
        lifecycle_generation: u64,
        disposed: bool,
        counters: WorthQueryArtifactLifecycleCounters,
    ) -> Self {
        Self {
            owner_count,
            borrow_count,
            lease_count,
            lifecycle_generation,
            disposed,
            counters,
        }
    }
}
