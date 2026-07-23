use std::{marker::PhantomData, sync::Arc};

use worth_store_physical_backend::{
    FilesystemBackendProfile, MediaCounterObserver, MediaCounterSnapshot, MutationOwnerObservation,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    lifecycle::{LifecycleGeneration, LifecycleState, ObservedLifecyclePhase},
    resource_lifecycle::ObservationLease,
    ObservationError, RuntimeCounterSnapshot, RuntimeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOwnedObservationPhase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordServingObservationPhase;

pub struct PhysicalMediaObserver<Phase> {
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    mutation_owner: MutationOwnerObservation,
    profile: Arc<FilesystemBackendProfile>,
    media_counters: MediaCounterObserver,
    lifecycle: Arc<LifecycleState>,
    observed_generation: LifecycleGeneration,
    expected_phase: ObservedLifecyclePhase,
    lease: ObservationLease,
    phase: PhantomData<Phase>,
}

struct MediaObservationSubject {
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    mutation_owner: MutationOwnerObservation,
}

impl PhysicalMediaObserver<MediaOwnedObservationPhase> {
    pub(in crate::physical_runtime) fn for_media_owned(
        runtime_identity: RuntimeIdentity,
        store_identity: StableStoreIdentity,
        mutation_owner: MutationOwnerObservation,
        profile: FilesystemBackendProfile,
        media_counters: MediaCounterObserver,
        lifecycle: Arc<LifecycleState>,
        lease: ObservationLease,
    ) -> Self {
        Self::new(
            MediaObservationSubject {
                runtime_identity,
                store_identity,
                mutation_owner,
            },
            profile,
            media_counters,
            lifecycle,
            ObservedLifecyclePhase::MediaOwned,
            lease,
        )
    }
}

impl PhysicalMediaObserver<RecordServingObservationPhase> {
    pub(in crate::physical_runtime) fn for_record_serving(
        runtime_identity: RuntimeIdentity,
        store_identity: StableStoreIdentity,
        mutation_owner: MutationOwnerObservation,
        profile: FilesystemBackendProfile,
        media_counters: MediaCounterObserver,
        lifecycle: Arc<LifecycleState>,
        lease: ObservationLease,
    ) -> Self {
        Self::new(
            MediaObservationSubject {
                runtime_identity,
                store_identity,
                mutation_owner,
            },
            profile,
            media_counters,
            lifecycle,
            ObservedLifecyclePhase::RecordServing,
            lease,
        )
    }
}

impl<Phase> PhysicalMediaObserver<Phase> {
    fn new(
        subject: MediaObservationSubject,
        profile: FilesystemBackendProfile,
        media_counters: MediaCounterObserver,
        lifecycle: Arc<LifecycleState>,
        expected_phase: ObservedLifecyclePhase,
        lease: ObservationLease,
    ) -> Self {
        let observed_generation = lifecycle.snapshot().generation;
        Self {
            runtime_identity: subject.runtime_identity,
            store_identity: subject.store_identity,
            mutation_owner: subject.mutation_owner,
            profile: Arc::new(profile),
            media_counters,
            lifecycle,
            observed_generation,
            expected_phase,
            lease,
            phase: PhantomData,
        }
    }

    pub fn snapshot(&self) -> Result<PhysicalMediaObservation<Phase>, ObservationError> {
        self.lease.counters().record_lifecycle_observation();
        let before = self.lifecycle.snapshot();
        if before.generation != self.observed_generation || before.phase != self.expected_phase {
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
            phase: PhantomData,
        })
    }

    pub fn media_counters(&self) -> MediaCounterSnapshot {
        self.media_counters.snapshot()
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.runtime_identity
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store_identity
    }

    pub(in crate::physical_runtime) const fn observed_generation(&self) -> LifecycleGeneration {
        self.observed_generation
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

impl<Phase> Clone for PhysicalMediaObserver<Phase> {
    fn clone(&self) -> Self {
        Self {
            runtime_identity: self.runtime_identity,
            store_identity: self.store_identity,
            mutation_owner: self.mutation_owner,
            profile: Arc::clone(&self.profile),
            media_counters: self.media_counters.clone(),
            lifecycle: Arc::clone(&self.lifecycle),
            observed_generation: self.observed_generation,
            expected_phase: self.expected_phase,
            lease: self.lease.clone(),
            phase: PhantomData,
        }
    }
}

pub struct PhysicalMediaObservation<Phase> {
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    mutation_owner: MutationOwnerObservation,
    profile: Arc<FilesystemBackendProfile>,
    media_counters: MediaCounterSnapshot,
    counters: RuntimeCounterSnapshot,
    generation: LifecycleGeneration,
    phase: PhantomData<Phase>,
}

impl<Phase> PhysicalMediaObservation<Phase> {
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
