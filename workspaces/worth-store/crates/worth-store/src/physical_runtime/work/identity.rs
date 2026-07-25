use std::num::NonZeroU64;

use worth_store_physical_backend::MediaOperationIdentity;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{LifecycleGeneration, RuntimeIdentity};

/// Monotonic identity allocated by one physical Store instance.
///
/// It is observable correlation, not permission to submit or execute work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalOperationIdentity(NonZeroU64);

impl PhysicalOperationIdentity {
    pub(in crate::physical_runtime) const fn from_owner_sequence(sequence: NonZeroU64) -> Self {
        Self(sequence)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// The lifecycle generation to which physical work is fenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWorkGeneration(LifecycleGeneration);

impl PhysicalWorkGeneration {
    pub(in crate::physical_runtime) const fn from_lifecycle(
        generation: LifecycleGeneration,
    ) -> Self {
        Self(generation)
    }

    pub const fn lifecycle(self) -> LifecycleGeneration {
        self.0
    }
}

/// Stable identity for one Store-owned physical work obligation.
///
/// Signal handles, scheduler bindings, digests, and backend receipts cannot
/// construct this value. Only the generation owner can combine the four
/// identity dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWorkIdentity {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: PhysicalWorkGeneration,
    operation: PhysicalOperationIdentity,
}

impl PhysicalWorkIdentity {
    pub(in crate::physical_runtime) const fn from_instance_owner(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        generation: PhysicalWorkGeneration,
        operation: PhysicalOperationIdentity,
    ) -> Self {
        Self {
            store,
            runtime,
            generation,
            operation,
        }
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn runtime(self) -> RuntimeIdentity {
        self.runtime
    }

    pub const fn generation(self) -> PhysicalWorkGeneration {
        self.generation
    }

    pub const fn operation(self) -> PhysicalOperationIdentity {
        self.operation
    }
}

/// Correlation between one Store work obligation and one backend effect
/// attempt. The backend operation identity remains verbatim evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalEffectIdentity {
    work: PhysicalWorkIdentity,
    backend_operation: MediaOperationIdentity,
}

impl PhysicalEffectIdentity {
    pub(in crate::physical_runtime) const fn new(
        work: PhysicalWorkIdentity,
        backend_operation: MediaOperationIdentity,
    ) -> Self {
        Self {
            work,
            backend_operation,
        }
    }

    pub const fn work(self) -> PhysicalWorkIdentity {
        self.work
    }

    pub const fn backend_operation(self) -> MediaOperationIdentity {
        self.backend_operation
    }
}
