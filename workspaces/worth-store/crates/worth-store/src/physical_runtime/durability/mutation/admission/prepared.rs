use super::super::{
    PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdempotencyLease,
    PhysicalMutationIdentity, PhysicalMutationRequestFingerprint,
    PhysicalMutationUnresolvedBindingObservation, UnallocatedPhysicalMutationAttemptBinding,
};

pub(in crate::physical_runtime) enum AdmittedPhysicalMutation {
    Fresh(UnallocatedPhysicalMutationAttemptBinding),
    DuplicateUnresolved {
        existing: PhysicalMutationUnresolvedBindingObservation,
        lease: PhysicalMutationIdempotencyLease,
    },
}

impl AdmittedPhysicalMutation {
    pub(in crate::physical_runtime) const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        match self {
            Self::Fresh(binding) => binding.mutation_identity(),
            Self::DuplicateUnresolved { existing, .. } => existing.mutation(),
        }
    }

    pub(in crate::physical_runtime) const fn idempotency_identity(
        &self,
    ) -> PhysicalMutationIdempotencyKeyIdentity {
        match self {
            Self::Fresh(binding) => binding.key().identity(),
            Self::DuplicateUnresolved { existing, .. } => existing.key(),
        }
    }

    pub(in crate::physical_runtime) const fn lease(&self) -> PhysicalMutationIdempotencyLease {
        match self {
            Self::Fresh(binding) => binding.key().lease(),
            Self::DuplicateUnresolved { lease, .. } => *lease,
        }
    }

    pub(in crate::physical_runtime) const fn fingerprint(
        &self,
    ) -> PhysicalMutationRequestFingerprint {
        match self {
            Self::Fresh(binding) => binding.fingerprint(),
            Self::DuplicateUnresolved { existing, .. } => existing.fingerprint(),
        }
    }

    pub(in crate::physical_runtime) const fn is_fresh(&self) -> bool {
        matches!(self, Self::Fresh(_))
    }

    pub(in crate::physical_runtime) fn into_fresh_binding(
        self,
    ) -> Option<UnallocatedPhysicalMutationAttemptBinding> {
        match self {
            Self::Fresh(binding) => Some(binding),
            Self::DuplicateUnresolved { .. } => None,
        }
    }
}
