use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{LifecycleGeneration, PhysicalWorkIdentity, RuntimeIdentity};

/// Identity fence shared by every capability in one C.6 physical-work handoff.
///
/// This is correlation and validation evidence, not submission or execution
/// authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct C6PhysicalWorkHandoffIdentity {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

impl C6PhysicalWorkHandoffIdentity {
    pub(in crate::physical_runtime) const fn new(
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

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn runtime(self) -> RuntimeIdentity {
        self.runtime
    }

    pub const fn generation(self) -> LifecycleGeneration {
        self.generation
    }

    pub fn admits(self, identity: PhysicalWorkIdentity) -> bool {
        identity.store() == self.store
            && identity.runtime() == self.runtime
            && identity.generation().lifecycle() == self.generation
    }
}
