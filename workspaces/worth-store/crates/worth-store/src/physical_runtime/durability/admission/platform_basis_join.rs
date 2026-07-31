use std::sync::Arc;

use worth_store_physical_backend::QualifiedFilesystemMedia;

use crate::physical_runtime::{
    durability::{
        PhysicalMutationIdempotencyRuntimeAuthority, PhysicalMutationIdempotencyRuntimeOwner,
    },
    RuntimeIdentity,
};

use super::AdmittedPhysicalDurabilityPolicy;

pub(in crate::physical_runtime) struct PhysicalDurabilityRuntimeOwner {
    policy: AdmittedPhysicalDurabilityPolicy,
    runtime: RuntimeIdentity,
    idempotency: Arc<PhysicalMutationIdempotencyRuntimeOwner>,
}

impl PhysicalDurabilityRuntimeOwner {
    pub(in crate::physical_runtime) const fn runtime_identity(&self) -> RuntimeIdentity {
        self.runtime
    }

    pub(in crate::physical_runtime) fn observation(
        &self,
    ) -> crate::physical_runtime::PhysicalDurabilityObservation {
        crate::physical_runtime::PhysicalDurabilityObservation::new(self.runtime, &self.policy)
    }

    pub(in crate::physical_runtime) fn idempotency_authority(
        &self,
    ) -> PhysicalMutationIdempotencyRuntimeAuthority {
        PhysicalMutationIdempotencyRuntimeOwner::authority(&self.idempotency)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::physical_runtime) enum PhysicalDurabilityRuntimeRebind {
    StoreIdentityMismatch,
    AdmissionBasisMismatch,
}

pub(in crate::physical_runtime) fn bind_policy_to_runtime(
    policy: AdmittedPhysicalDurabilityPolicy,
    media: &QualifiedFilesystemMedia,
    runtime: RuntimeIdentity,
) -> Result<PhysicalDurabilityRuntimeOwner, PhysicalDurabilityRuntimeRebind> {
    if policy.store_identity() != media.store_identity() {
        return Err(PhysicalDurabilityRuntimeRebind::StoreIdentityMismatch);
    }
    let current = media
        .physical_durability_admission_identity()
        .map_err(|_| PhysicalDurabilityRuntimeRebind::AdmissionBasisMismatch)?;
    if policy.admission_basis_identity() != current {
        return Err(PhysicalDurabilityRuntimeRebind::AdmissionBasisMismatch);
    }
    let idempotency_policy = policy.idempotency_policy();
    let idempotency = PhysicalMutationIdempotencyRuntimeOwner::generation_zero(
        policy.store_identity(),
        runtime,
        policy.identity(),
        idempotency_policy,
    );
    Ok(PhysicalDurabilityRuntimeOwner {
        policy,
        runtime,
        idempotency,
    })
}
