mod attempt_binding;
mod binding_compaction;
mod bootstrap;
mod fate;
mod key;
mod lease;
mod persisted_binding;
mod registry;
mod runtime_owner;
#[cfg(test)]
mod test_support;

pub(in crate::physical_runtime) use attempt_binding::{
    AllocatedPhysicalMutationAttemptBinding, UnallocatedPhysicalMutationAttemptBinding,
};
pub use binding_compaction::PhysicalMutationBindingCompaction;
pub use bootstrap::PhysicalIdempotencyReopenFailure;
pub(in crate::physical_runtime) use bootstrap::{
    rebuild_idempotency, RebuiltPhysicalMutationIdempotency,
};
pub use key::{
    PhysicalMutationIdempotencyKey, PhysicalMutationIdempotencyKeyIdentity,
    PhysicalMutationIdempotencyMaterial,
};
pub use lease::{PhysicalMutationIdempotencyLease, PhysicalNamespaceDurableCheckpointGeneration};
pub(in crate::physical_runtime) use persisted_binding::PersistedPhysicalMutationAttemptBinding;
pub(in crate::physical_runtime) use registry::{
    PhysicalMutationGroupSealingBinding, PhysicalMutationIdempotencyGroupSealDenial,
    PhysicalMutationIdempotencyRegistryAdmission,
    PhysicalMutationIdempotencyRegistryAdmissionError, PhysicalMutationIdempotencyRegistryDenial,
    PhysicalMutationPreSealCancellationDenial, PhysicalMutationTerminalizationDenial,
    PhysicalMutationUnresolvedBindingObservation,
};
pub use runtime_owner::PhysicalMutationIdempotencyIssuanceDenial;
pub(in crate::physical_runtime) use runtime_owner::{
    PhysicalMutationBindingCompactionCutover, PhysicalMutationBindingCompactionRuntimeAuthority,
    PhysicalMutationIdempotencyRuntimeAuthority, PhysicalMutationIdempotencyRuntimeOwner,
};
