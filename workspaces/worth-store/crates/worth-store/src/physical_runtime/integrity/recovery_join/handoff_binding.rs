use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_integrity::PhysicalIntegrityObservationOutcome;

use super::RecoveryIntegrityRuntimeGeneration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum RecoveryIntegrityHandoffDenial {
    StoreMismatch,
    RootGenerationMismatch,
}

/// Descriptive integrity observations bound to the Store handoff generation.
///
/// This value does not preserve recovery admission or open a decoder. It keeps
/// observations from being silently attached to another Store/runtime join.
pub(in crate::physical_runtime) struct RecoveryIntegrityHandoffBinding {
    store: StableStoreIdentity,
    root_generation: u64,
    runtime_generation: RecoveryIntegrityRuntimeGeneration,
    observations: Vec<PhysicalIntegrityObservationOutcome>,
}

impl RecoveryIntegrityHandoffBinding {
    pub(in crate::physical_runtime) fn bind(
        store: StableStoreIdentity,
        root_generation: u64,
        runtime_generation: RecoveryIntegrityRuntimeGeneration,
        observations: Vec<PhysicalIntegrityObservationOutcome>,
    ) -> Result<Self, RecoveryIntegrityHandoffDenial> {
        for observation in &observations {
            let scope = observation.scope();
            if scope.store_identity() != store {
                return Err(RecoveryIntegrityHandoffDenial::StoreMismatch);
            }
            if scope
                .root_generation()
                .is_some_and(|generation| generation != root_generation)
            {
                return Err(RecoveryIntegrityHandoffDenial::RootGenerationMismatch);
            }
        }
        Ok(Self {
            store,
            root_generation,
            runtime_generation,
            observations,
        })
    }

    #[cfg(feature = "recovery-runtime-owner")]
    pub(in crate::physical_runtime) fn from_recovered_core(
        core: &crate::physical_runtime::RecoveredPhysicalRuntimeCore,
        lifecycle_generation: crate::physical_runtime::LifecycleGeneration,
        observations: Vec<PhysicalIntegrityObservationOutcome>,
    ) -> Result<Self, RecoveryIntegrityHandoffDenial> {
        Self::bind(
            core.store_identity(),
            core.root().generation(),
            RecoveryIntegrityRuntimeGeneration::bind(lifecycle_generation),
            observations,
        )
    }

    pub(in crate::physical_runtime) const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub(in crate::physical_runtime) const fn root_generation(&self) -> u64 {
        self.root_generation
    }

    pub(in crate::physical_runtime) const fn runtime_generation(
        &self,
    ) -> RecoveryIntegrityRuntimeGeneration {
        self.runtime_generation
    }

    pub(in crate::physical_runtime) fn observations(
        &self,
    ) -> &[PhysicalIntegrityObservationOutcome] {
        &self.observations
    }
}
