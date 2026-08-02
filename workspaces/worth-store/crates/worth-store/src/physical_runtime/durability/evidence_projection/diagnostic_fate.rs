use crate::physical_runtime::{
    IndeterminatePhysicalMutation, PhysicalMutationIdempotencyKeyIdentity,
    PhysicalMutationIdentity, PhysicalMutationIndeterminateStage,
    PhysicalMutationProvenNoEffectCause, PhysicalMutationRequestFingerprint,
    ProvenNoEffectPhysicalMutation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvenNoEffectPhysicalMutationEvidence {
    mutation: PhysicalMutationIdentity,
    idempotency: PhysicalMutationIdempotencyKeyIdentity,
    fingerprint: PhysicalMutationRequestFingerprint,
    cause: PhysicalMutationProvenNoEffectCause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminatePhysicalMutationEvidence {
    mutation: PhysicalMutationIdentity,
    idempotency: PhysicalMutationIdempotencyKeyIdentity,
    fingerprint: PhysicalMutationRequestFingerprint,
    stage: PhysicalMutationIndeterminateStage,
    completed_effects: u32,
}

impl ProvenNoEffectPhysicalMutationEvidence {
    pub(in crate::physical_runtime) const fn from_fate(
        fate: ProvenNoEffectPhysicalMutation,
    ) -> Self {
        Self {
            mutation: fate.mutation_identity(),
            idempotency: fate.idempotency_identity(),
            fingerprint: fate.request_fingerprint(),
            cause: fate.cause(),
        }
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.mutation
    }
    pub const fn idempotency_identity(self) -> PhysicalMutationIdempotencyKeyIdentity {
        self.idempotency
    }
    pub const fn request_fingerprint(self) -> PhysicalMutationRequestFingerprint {
        self.fingerprint
    }
    pub const fn cause(self) -> PhysicalMutationProvenNoEffectCause {
        self.cause
    }
}

impl IndeterminatePhysicalMutationEvidence {
    pub(in crate::physical_runtime) const fn from_fate(
        fate: IndeterminatePhysicalMutation,
    ) -> Self {
        Self {
            mutation: fate.mutation_identity(),
            idempotency: fate.idempotency_identity(),
            fingerprint: fate.request_fingerprint(),
            stage: fate.stage(),
            completed_effects: fate.completed_effect_count(),
        }
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.mutation
    }
    pub const fn idempotency_identity(self) -> PhysicalMutationIdempotencyKeyIdentity {
        self.idempotency
    }
    pub const fn request_fingerprint(self) -> PhysicalMutationRequestFingerprint {
        self.fingerprint
    }
    pub const fn stage(self) -> PhysicalMutationIndeterminateStage {
        self.stage
    }
    pub const fn completed_effect_count(self) -> u32 {
        self.completed_effects
    }
}
