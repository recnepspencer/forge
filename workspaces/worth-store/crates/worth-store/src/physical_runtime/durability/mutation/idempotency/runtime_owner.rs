use std::sync::{Arc, Mutex, Weak};

use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    PhysicalDurabilityPolicyIdentity, PhysicalIdempotencyPolicy, RuntimeIdentity,
};

use super::{
    lease::PhysicalMutationLeaseIssuanceFailure,
    registry::{
        PhysicalMutationIdempotencyRegistry, PhysicalMutationIdempotencyRegistryAdmission,
        PhysicalMutationIdempotencyRegistryAdmissionError,
        PhysicalMutationIdempotencyRegistryDenial,
    },
    PhysicalMutationIdempotencyKey, PhysicalMutationIdempotencyMaterial,
};

pub(in crate::physical_runtime) struct PhysicalMutationIdempotencyRuntimeOwner {
    registry: Mutex<PhysicalMutationIdempotencyRegistry>,
}

#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalMutationIdempotencyRuntimeAuthority {
    owner: Weak<PhysicalMutationIdempotencyRuntimeOwner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalMutationIdempotencyIssuanceDenial {
    DurabilityAuthorityReleased,
    LeaseGenerationExhausted,
}

impl PhysicalMutationIdempotencyRuntimeOwner {
    pub(in crate::physical_runtime) fn generation_zero(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        policy: PhysicalDurabilityPolicyIdentity,
        idempotency: PhysicalIdempotencyPolicy,
    ) -> Arc<Self> {
        Arc::new(Self {
            registry: Mutex::new(PhysicalMutationIdempotencyRegistry::generation_zero(
                store,
                runtime,
                policy,
                idempotency,
            )),
        })
    }

    pub(in crate::physical_runtime) fn authority(
        owner: &Arc<Self>,
    ) -> PhysicalMutationIdempotencyRuntimeAuthority {
        PhysicalMutationIdempotencyRuntimeAuthority {
            owner: Arc::downgrade(owner),
        }
    }
}

impl PhysicalMutationIdempotencyRuntimeAuthority {
    pub(in crate::physical_runtime) fn issue_key(
        &self,
        material: PhysicalMutationIdempotencyMaterial,
    ) -> Result<PhysicalMutationIdempotencyKey, PhysicalMutationIdempotencyIssuanceDenial> {
        let owner = self
            .owner
            .upgrade()
            .ok_or(PhysicalMutationIdempotencyIssuanceDenial::DurabilityAuthorityReleased)?;
        let registry = owner
            .registry
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let issued = registry
            .issue_key(material)
            .map_err(|failure| match failure {
                PhysicalMutationLeaseIssuanceFailure::GenerationExhausted => {
                    PhysicalMutationIdempotencyIssuanceDenial::LeaseGenerationExhausted
                }
            });
        issued
    }

    pub(in crate::physical_runtime) fn admit_unallocated_with<E>(
        &self,
        key: PhysicalMutationIdempotencyKey,
        fingerprint: crate::physical_runtime::PhysicalMutationRequestFingerprint,
        reserve: impl FnOnce() -> Result<crate::physical_runtime::PhysicalMutationIdentity, E>,
    ) -> Result<
        PhysicalMutationIdempotencyRegistryAdmission,
        PhysicalMutationIdempotencyRegistryAdmissionError<E>,
    > {
        let owner = self.owner.upgrade().ok_or(
            PhysicalMutationIdempotencyRegistryAdmissionError::Denied(
                PhysicalMutationIdempotencyRegistryDenial::AuthorityReleased,
            ),
        )?;
        let admission = owner
            .registry
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .admit_unallocated_with(key, fingerprint, reserve);
        admission
    }
}
