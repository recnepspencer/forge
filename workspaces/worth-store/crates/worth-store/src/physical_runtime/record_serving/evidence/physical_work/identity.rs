use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{LifecycleGeneration, RuntimeIdentity};

/// Correlation fence for one physical-work courtroom observation.
///
/// This is validation evidence only. It cannot submit work, reach residency,
/// or authorize execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhysicalWorkCourtroomIdentity {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

impl PhysicalWorkCourtroomIdentity {
    pub(super) const fn new(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        generation: LifecycleGeneration,
    ) -> Self {
        Self {
            store,
            runtime,
            generation,
        }
    }

    pub(super) const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub(super) const fn runtime(self) -> RuntimeIdentity {
        self.runtime
    }

    pub(super) const fn generation(self) -> LifecycleGeneration {
        self.generation
    }
}
