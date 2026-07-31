use std::collections::BTreeMap;

use crate::physical_runtime::{
    PendingUnresolvedMutationLimit, PhysicalDurabilityPolicyIdentity, PhysicalIdempotencyPolicy,
    RuntimeIdentity,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::super::{PhysicalMutationIdentity, PhysicalMutationRequestFingerprint};
use super::{
    attempt_binding::{
        PhysicalMutationAttemptBinding, UnallocatedPhysicalMutationAttemptBinding, WalUnallocated,
    },
    lease::{PhysicalMutationLeaseIssuanceFailure, PhysicalNamespaceDurableCheckpointGeneration},
    PhysicalMutationIdempotencyKey, PhysicalMutationIdempotencyKeyIdentity,
    PhysicalMutationIdempotencyLease, PhysicalMutationIdempotencyMaterial,
};

pub(super) struct PhysicalMutationIdempotencyRegistry {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    policy: PhysicalDurabilityPolicyIdentity,
    retention: crate::physical_runtime::IdempotencyRetentionGenerations,
    pending_limit: PendingUnresolvedMutationLimit,
    generation: PhysicalNamespaceDurableCheckpointGeneration,
    unresolved: BTreeMap<
        PhysicalMutationIdempotencyKeyIdentity,
        PhysicalMutationUnresolvedBindingObservation,
    >,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PhysicalMutationUnresolvedBindingObservation {
    key: PhysicalMutationIdempotencyKeyIdentity,
    fingerprint: PhysicalMutationRequestFingerprint,
    mutation: PhysicalMutationIdentity,
}

pub(in crate::physical_runtime) enum PhysicalMutationIdempotencyRegistryAdmission {
    Fresh(UnallocatedPhysicalMutationAttemptBinding),
    DuplicateUnresolved(PhysicalMutationUnresolvedBindingObservation),
}

pub(in crate::physical_runtime) enum PhysicalMutationIdempotencyRegistryAdmissionError<E> {
    Denied(PhysicalMutationIdempotencyRegistryDenial),
    Reservation(E),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum PhysicalMutationIdempotencyRegistryDenial {
    AuthorityReleased,
    ForeignStore,
    ForeignPolicy,
    ForeignMutationStore,
    ForeignMutationRuntime,
    Expired,
    Conflict,
    PendingUnresolvedLimitReached,
}

impl PhysicalMutationUnresolvedBindingObservation {
    pub(in crate::physical_runtime) const fn key(self) -> PhysicalMutationIdempotencyKeyIdentity {
        self.key
    }

    pub(in crate::physical_runtime) const fn fingerprint(
        self,
    ) -> PhysicalMutationRequestFingerprint {
        self.fingerprint
    }

    pub(in crate::physical_runtime) const fn mutation(self) -> PhysicalMutationIdentity {
        self.mutation
    }
}

impl PhysicalMutationIdempotencyRegistry {
    pub(super) fn generation_zero(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        policy: PhysicalDurabilityPolicyIdentity,
        idempotency: PhysicalIdempotencyPolicy,
    ) -> Self {
        Self {
            store,
            runtime,
            policy,
            retention: idempotency.retention(),
            pending_limit: idempotency.pending_unresolved_limit(),
            generation: PhysicalNamespaceDurableCheckpointGeneration::INITIAL,
            unresolved: BTreeMap::new(),
        }
    }

    pub(super) fn issue_key(
        &self,
        material: PhysicalMutationIdempotencyMaterial,
    ) -> Result<PhysicalMutationIdempotencyKey, PhysicalMutationLeaseIssuanceFailure> {
        let lease = PhysicalMutationIdempotencyLease::issue(
            self.store,
            self.policy,
            self.generation,
            self.retention,
        )?;
        Ok(PhysicalMutationIdempotencyKey::issue(lease, material))
    }

    #[cfg(test)]
    pub(super) fn admit_unallocated(
        &mut self,
        key: PhysicalMutationIdempotencyKey,
        fingerprint: PhysicalMutationRequestFingerprint,
        mutation: PhysicalMutationIdentity,
    ) -> Result<
        PhysicalMutationIdempotencyRegistryAdmission,
        PhysicalMutationIdempotencyRegistryDenial,
    > {
        match self.admit_unallocated_with(key, fingerprint, || {
            Ok::<_, std::convert::Infallible>(mutation)
        }) {
            Ok(admission) => Ok(admission),
            Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(denial)) => Err(denial),
            Err(PhysicalMutationIdempotencyRegistryAdmissionError::Reservation(never)) => {
                match never {}
            }
        }
    }

    pub(in crate::physical_runtime) fn admit_unallocated_with<E>(
        &mut self,
        key: PhysicalMutationIdempotencyKey,
        fingerprint: PhysicalMutationRequestFingerprint,
        reserve: impl FnOnce() -> Result<PhysicalMutationIdentity, E>,
    ) -> Result<
        PhysicalMutationIdempotencyRegistryAdmission,
        PhysicalMutationIdempotencyRegistryAdmissionError<E>,
    > {
        if key.lease().store_identity() != self.store {
            return Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::ForeignStore,
            ));
        }
        if key.lease().policy_identity() != self.policy {
            return Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::ForeignPolicy,
            ));
        }
        if let Some(existing) = self.unresolved.get(&key.identity()) {
            return if existing.fingerprint == fingerprint {
                Ok(PhysicalMutationIdempotencyRegistryAdmission::DuplicateUnresolved(*existing))
            } else {
                Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                    PhysicalMutationIdempotencyRegistryDenial::Conflict,
                ))
            };
        }
        if key.lease().is_expired_at(self.generation) {
            return Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::Expired,
            ));
        }
        if self.unresolved.len() >= self.pending_limit.get().get() as usize {
            return Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::PendingUnresolvedLimitReached,
            ));
        }
        let mutation =
            reserve().map_err(PhysicalMutationIdempotencyRegistryAdmissionError::Reservation)?;
        if mutation.store_identity() != self.store {
            return Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::ForeignMutationStore,
            ));
        }
        if mutation.runtime_identity() != self.runtime {
            return Err(PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::ForeignMutationRuntime,
            ));
        }
        let observation = PhysicalMutationUnresolvedBindingObservation {
            key: key.identity(),
            fingerprint,
            mutation,
        };
        self.unresolved.insert(key.identity(), observation);
        Ok(PhysicalMutationIdempotencyRegistryAdmission::Fresh(
            PhysicalMutationAttemptBinding::<WalUnallocated>::new(key, fingerprint, mutation),
        ))
    }

    #[cfg(test)]
    pub(super) fn set_namespace_durable_generation_for_test(&mut self, generation: u64) {
        self.generation =
            PhysicalNamespaceDurableCheckpointGeneration::from_namespace_durable_checkpoint(
                generation,
            );
    }
}

#[cfg(test)]
#[path = "registry/tests.rs"]
mod tests;
