use worth_store_security::StoreAuthorityBoundSecurityScopeReceipt;

use super::{
    PhysicalSignalAspectBindingDigest, PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass,
    PhysicalWorkGeneration, PhysicalWorkIntent, PhysicalWorkOperationFamily,
    PhysicalWorkSignalFamily,
};

/// Borrow-free proof that admission is being performed by the live owner of
/// one qualified physical Store instance.
///
/// Construction requires the qualified media object and is confined to the
/// physical composition root. The proof grants no effect method; it only
/// supplies the exact physical owner facts that admission must bind.
#[derive(Clone, Copy)]
pub(in crate::physical_runtime) struct PhysicalWorkAdmissionAuthority {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    runtime: crate::physical_runtime::RuntimeIdentity,
    generation: crate::physical_runtime::LifecycleGeneration,
    media_owner: worth_store_physical_backend::MutationOwnerObservation,
}

/// Exact physical facts sealed by the live Store instance before Signal admission.
///
/// This value is not effect authority. It proves that the immutable work packet
/// belongs to the current Store/runtime generation, is confined to the Store's
/// artifact namespace, and carries an admitted security scope and installed
/// semantic binding.
#[derive(Debug)]
pub struct AdmittedPhysicalWorkAuthority {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    runtime: crate::physical_runtime::RuntimeIdentity,
    generation: PhysicalWorkGeneration,
    scope_digest: [u8; 32],
    security: StoreAuthorityBoundSecurityScopeReceipt,
    binding: PhysicalSignalAspectBindingDigest,
    signal_family: PhysicalWorkSignalFamily,
    operation: PhysicalWorkOperationFamily,
    effect: PhysicalWorkEffectClass,
    durability: PhysicalWorkDurabilityRequirement,
    media_owner: worth_store_physical_backend::MutationOwnerObservation,
}

impl PhysicalWorkAdmissionAuthority {
    pub(in crate::physical_runtime) fn from_qualified_instance(
        media: &worth_store_physical_backend::QualifiedFilesystemMedia,
        runtime: crate::physical_runtime::RuntimeIdentity,
        generation: crate::physical_runtime::LifecycleGeneration,
    ) -> Self {
        Self {
            store: media.store_identity(),
            runtime,
            generation,
            media_owner: media.mutation_owner(),
        }
    }

    pub(super) const fn store(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    pub(super) const fn runtime(&self) -> crate::physical_runtime::RuntimeIdentity {
        self.runtime
    }

    pub(super) const fn generation(&self) -> crate::physical_runtime::LifecycleGeneration {
        self.generation
    }
}

impl AdmittedPhysicalWorkAuthority {
    pub(super) fn seal(
        intent: &PhysicalWorkIntent,
        binding: PhysicalSignalAspectBindingDigest,
        signal_family: PhysicalWorkSignalFamily,
        physical: &PhysicalWorkAdmissionAuthority,
    ) -> Self {
        Self {
            store: intent.identity().store(),
            runtime: intent.identity().runtime(),
            generation: intent.identity().generation(),
            scope_digest: intent.scope().stable_digest(),
            security: intent.security_authority(),
            binding,
            signal_family,
            operation: intent.operation(),
            effect: intent.effect(),
            durability: intent.durability(),
            media_owner: physical.media_owner,
        }
    }

    pub const fn store(&self) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.store
    }

    pub const fn runtime(&self) -> crate::physical_runtime::RuntimeIdentity {
        self.runtime
    }

    pub const fn generation(&self) -> PhysicalWorkGeneration {
        self.generation
    }

    pub const fn scope_digest(&self) -> &[u8; 32] {
        &self.scope_digest
    }

    pub const fn security(&self) -> StoreAuthorityBoundSecurityScopeReceipt {
        self.security
    }

    pub const fn binding(&self) -> PhysicalSignalAspectBindingDigest {
        self.binding
    }

    pub const fn signal_family(&self) -> PhysicalWorkSignalFamily {
        self.signal_family
    }

    pub const fn operation(&self) -> PhysicalWorkOperationFamily {
        self.operation
    }

    pub const fn effect(&self) -> PhysicalWorkEffectClass {
        self.effect
    }

    pub const fn durability(&self) -> PhysicalWorkDurabilityRequirement {
        self.durability
    }

    pub const fn media_owner_observation(
        &self,
    ) -> worth_store_physical_backend::MutationOwnerObservation {
        self.media_owner
    }
}
