use crate::physical_runtime::{
    PhysicalDurabilityGroupMemberBinding, PhysicalDurabilityPolicyIdentity,
    PhysicalMutationAcknowledgment, PhysicalMutationCompletedBreadth,
    PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdentity,
    PhysicalMutationRequestFingerprint,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationExecutedBoundaryEvidence {
    mutation: PhysicalMutationIdentity,
    idempotency: PhysicalMutationIdempotencyKeyIdentity,
    fingerprint: PhysicalMutationRequestFingerprint,
    group: PhysicalDurabilityGroupMemberBinding,
    policy: PhysicalDurabilityPolicyIdentity,
    breadth: PhysicalMutationCompletedBreadth,
}

impl PhysicalMutationExecutedBoundaryEvidence {
    pub(in crate::physical_runtime) fn from_acknowledgment(
        acknowledgment: &PhysicalMutationAcknowledgment,
    ) -> Self {
        Self {
            mutation: acknowledgment.mutation_identity(),
            idempotency: acknowledgment.idempotency_identity(),
            fingerprint: acknowledgment.request_fingerprint(),
            group: acknowledgment.group_binding(),
            policy: acknowledgment.durability_policy_identity(),
            breadth: acknowledgment.completed_breadth(),
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
    pub const fn group_binding(self) -> PhysicalDurabilityGroupMemberBinding {
        self.group
    }
    pub const fn durability_policy_identity(self) -> PhysicalDurabilityPolicyIdentity {
        self.policy
    }
    pub const fn completed_breadth(self) -> PhysicalMutationCompletedBreadth {
        self.breadth
    }
}
