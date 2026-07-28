use worth_store::physical_runtime::{LifecycleGeneration, PhysicalResidencyObservation};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

/// Store-owned identity retained by admitted physical counter evidence.
///
/// The only constructor consumes Store's sealed residency observation, so
/// synthetic counter rows cannot claim a production residency source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyEvidenceSource {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
}

impl PhysicalResidencyEvidenceSource {
    pub(crate) const fn from_store_observation(observation: PhysicalResidencyObservation) -> Self {
        Self {
            store: observation.store_identity(),
            generation: observation.store_generation(),
        }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn store_generation(self) -> LifecycleGeneration {
        self.generation
    }
}
