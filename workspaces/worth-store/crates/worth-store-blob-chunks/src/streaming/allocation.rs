use worth_store::physical_runtime::{BlobPhysicalAllocation, LifecycleGeneration, RuntimeIdentity};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

#[derive(Debug)]
pub(crate) struct AdmittedBlobStreamingAllocation<'runtime> {
    allocation: BlobPhysicalAllocation<'runtime>,
    observation: BlobStreamingAllocationObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingAllocationObservation {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
    runtime: RuntimeIdentity,
    allocation_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlobStreamingAllocationDenial {
    WindowExceedsAllocation {
        window_bytes: u64,
        allocation_bytes: u64,
    },
}

impl<'runtime> AdmittedBlobStreamingAllocation<'runtime> {
    pub(crate) fn admit(
        allocation: BlobPhysicalAllocation<'runtime>,
        window_bytes: u64,
    ) -> Result<Self, BlobStreamingAllocationDenial> {
        if allocation.bytes() < window_bytes {
            return Err(BlobStreamingAllocationDenial::WindowExceedsAllocation {
                window_bytes,
                allocation_bytes: allocation.bytes(),
            });
        }
        let observation = BlobStreamingAllocationObservation {
            store: allocation.store_identity(),
            generation: allocation.store_generation(),
            runtime: allocation.runtime_identity(),
            allocation_bytes: allocation.bytes(),
        };
        Ok(Self {
            allocation,
            observation,
        })
    }

    pub(crate) const fn observation(&self) -> BlobStreamingAllocationObservation {
        self.observation
    }

    pub(crate) const fn bytes(&self) -> u64 {
        self.allocation.bytes()
    }
}

impl BlobStreamingAllocationObservation {
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
}
