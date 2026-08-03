use worth_store::physical_runtime::{
    LifecycleGeneration, RecoveryPhysicalAllocation, RuntimeIdentity,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

#[derive(Debug)]
pub struct RecoveryMemoryAllocation<'runtime> {
    allocation: RecoveryPhysicalAllocation<'runtime>,
    observation: RecoveryMemoryObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryObservation {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
    runtime: RuntimeIdentity,
    allocation_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryMemoryCounterSnapshot {
    allocation_bytes: u64,
}

impl<'runtime> RecoveryMemoryAllocation<'runtime> {
    pub fn from_store_allocation(allocation: RecoveryPhysicalAllocation<'runtime>) -> Self {
        let observation = RecoveryMemoryObservation {
            store: allocation.store_identity(),
            generation: allocation.store_generation(),
            runtime: allocation.runtime_identity(),
            allocation_bytes: allocation.bytes(),
        };
        Self {
            allocation,
            observation,
        }
    }

    pub const fn bytes(&self) -> u64 {
        self.allocation.bytes()
    }

    pub const fn observation(&self) -> RecoveryMemoryObservation {
        self.observation
    }

    pub const fn counters(&self) -> RecoveryMemoryCounterSnapshot {
        RecoveryMemoryCounterSnapshot {
            allocation_bytes: self.allocation.bytes(),
        }
    }
}

impl RecoveryMemoryObservation {
    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn store_generation(self) -> LifecycleGeneration {
        self.generation
    }

    pub const fn runtime_identity(self) -> RuntimeIdentity {
        self.runtime
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn counters(self) -> RecoveryMemoryCounterSnapshot {
        RecoveryMemoryCounterSnapshot {
            allocation_bytes: self.allocation_bytes,
        }
    }
}

impl RecoveryMemoryCounterSnapshot {
    pub const fn admitted(self) -> u32 {
        1
    }

    pub const fn resident_bytes_admitted(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn resident_frames_admitted(self) -> u32 {
        0
    }

    pub const fn allocation_bytes_allocated(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn pinned_pages_admitted(self) -> u32 {
        0
    }

    pub const fn copied_bytes(self) -> u64 {
        0
    }
}
