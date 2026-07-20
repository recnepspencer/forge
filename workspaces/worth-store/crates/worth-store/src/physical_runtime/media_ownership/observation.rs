use std::sync::Arc;

use worth_store_physical_backend::{
    FilesystemBackendProfile, MediaCounterObserver, MediaCounterSnapshot, MutationOwnerObservation,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    lifecycle::{LifecycleGeneration, LifecycleState, ObservedLifecyclePhase},
    resource_lifecycle::ObservationLease,
    ObservationError, RuntimeCounterSnapshot, RuntimeIdentity,
};

pub struct PhysicalMediaObserver {
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    mutation_owner: MutationOwnerObservation,
    profile: Arc<FilesystemBackendProfile>,
    media_counters: MediaCounterObserver,
    lifecycle: Arc<LifecycleState>,
    observed_generation: LifecycleGeneration,
    lease: ObservationLease,
}

impl PhysicalMediaObserver {
    pub(super) fn new(
        runtime_identity: RuntimeIdentity,
        store_identity: StableStoreIdentity,
        mutation_owner: MutationOwnerObservation,
        profile: FilesystemBackendProfile,
        media_counters: MediaCounterObserver,
        lifecycle: Arc<LifecycleState>,
        lease: ObservationLease,
    ) -> Self {
        let observed_generation = lifecycle.snapshot().generation;
        Self {
            runtime_identity,
            store_identity,
            mutation_owner,
            profile: Arc::new(profile),
            media_counters,
            lifecycle,
            observed_generation,
            lease,
        }
    }

    pub fn snapshot(&self) -> Result<PhysicalMediaObservation, ObservationError> {
        self.lease.counters().record_lifecycle_observation();
        let before = self.lifecycle.snapshot();
        if before.generation != self.observed_generation
            || before.phase != ObservedLifecyclePhase::MediaOwned
        {
            return Err(self.classify(before.phase, before.generation));
        }
        let media_counters = self.media_counters.snapshot();
        let counters = self.lease.counters().snapshot(before.generation);
        let after = self.lifecycle.snapshot();
        if after != before {
            return Err(self.classify(after.phase, after.generation));
        }
        Ok(PhysicalMediaObservation {
            runtime_identity: self.runtime_identity,
            store_identity: self.store_identity,
            mutation_owner: self.mutation_owner,
            profile: Arc::clone(&self.profile),
            media_counters,
            counters,
            generation: after.generation,
        })
    }

    pub fn media_counters(&self) -> MediaCounterSnapshot {
        self.media_counters.snapshot()
    }

    pub fn runtime_counters(&self) -> RuntimeCounterSnapshot {
        let lifecycle = self.lifecycle.snapshot();
        self.lease.counters().snapshot(lifecycle.generation)
    }

    fn classify(
        &self,
        phase: ObservedLifecyclePhase,
        generation: LifecycleGeneration,
    ) -> ObservationError {
        if phase == ObservedLifecyclePhase::Closed {
            ObservationError::Closed {
                runtime_identity: self.runtime_identity,
                closed_generation: generation,
            }
        } else {
            ObservationError::Stale {
                runtime_identity: self.runtime_identity,
                observed_generation: self.observed_generation,
                current_generation: generation,
            }
        }
    }
}

impl Clone for PhysicalMediaObserver {
    fn clone(&self) -> Self {
        Self {
            runtime_identity: self.runtime_identity,
            store_identity: self.store_identity,
            mutation_owner: self.mutation_owner,
            profile: Arc::clone(&self.profile),
            media_counters: self.media_counters.clone(),
            lifecycle: Arc::clone(&self.lifecycle),
            observed_generation: self.observed_generation,
            lease: self.lease.clone(),
        }
    }
}

pub struct PhysicalMediaObservation {
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    mutation_owner: MutationOwnerObservation,
    profile: Arc<FilesystemBackendProfile>,
    media_counters: MediaCounterSnapshot,
    counters: RuntimeCounterSnapshot,
    generation: LifecycleGeneration,
}

impl PhysicalMediaObservation {
    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.runtime_identity
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store_identity
    }

    pub const fn mutation_owner(&self) -> MutationOwnerObservation {
        self.mutation_owner
    }

    pub fn backend_profile(&self) -> &FilesystemBackendProfile {
        &self.profile
    }

    pub const fn media_counters(&self) -> MediaCounterSnapshot {
        self.media_counters
    }

    pub const fn counters(&self) -> RuntimeCounterSnapshot {
        self.counters
    }

    pub const fn generation(&self) -> LifecycleGeneration {
        self.generation
    }
}
