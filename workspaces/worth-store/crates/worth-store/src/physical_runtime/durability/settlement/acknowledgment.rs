use worth_store_physical_format::PersistedRecordIdentity;

use crate::physical_runtime::{
    PhysicalDurabilityGroupMemberBinding, PhysicalDurabilityPolicyIdentity,
    PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdentity,
    PhysicalMutationRequestFingerprint, PhysicalWalMemberBasis, RecordAppendObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationCompletedBreadth {
    record_count: u32,
    data_effect_count: u32,
    wal_member: PhysicalWalMemberBasis,
    current_root_generation: u64,
}

pub struct PhysicalMutationAcknowledgment {
    mutation: PhysicalMutationIdentity,
    idempotency: PhysicalMutationIdempotencyKeyIdentity,
    fingerprint: PhysicalMutationRequestFingerprint,
    group_binding: PhysicalDurabilityGroupMemberBinding,
    policy: PhysicalDurabilityPolicyIdentity,
    breadth: PhysicalMutationCompletedBreadth,
    records: Box<[PersistedRecordIdentity]>,
    observation: RecordAppendObservation,
}

pub(in crate::physical_runtime) struct PhysicalMutationAcknowledgmentBasis {
    pub(in crate::physical_runtime) mutation: PhysicalMutationIdentity,
    pub(in crate::physical_runtime) idempotency: PhysicalMutationIdempotencyKeyIdentity,
    pub(in crate::physical_runtime) fingerprint: PhysicalMutationRequestFingerprint,
    pub(in crate::physical_runtime) group_binding: PhysicalDurabilityGroupMemberBinding,
    pub(in crate::physical_runtime) policy: PhysicalDurabilityPolicyIdentity,
    pub(in crate::physical_runtime) breadth: PhysicalMutationCompletedBreadth,
    pub(in crate::physical_runtime) records: Box<[PersistedRecordIdentity]>,
    pub(in crate::physical_runtime) observation: RecordAppendObservation,
}

impl PhysicalMutationCompletedBreadth {
    pub(in crate::physical_runtime) fn completed(
        records: usize,
        data_effects: usize,
        wal_member: PhysicalWalMemberBasis,
        current_root_generation: u64,
    ) -> Self {
        Self {
            record_count: u32::try_from(records).expect("admitted physical record count fits u32"),
            data_effect_count: u32::try_from(data_effects)
                .expect("admitted physical data-effect count fits u32"),
            wal_member,
            current_root_generation,
        }
    }

    pub const fn record_count(self) -> u32 {
        self.record_count
    }

    pub const fn data_effect_count(self) -> u32 {
        self.data_effect_count
    }

    pub const fn wal_member(self) -> PhysicalWalMemberBasis {
        self.wal_member
    }

    pub const fn current_root_generation(self) -> u64 {
        self.current_root_generation
    }
}

impl PhysicalMutationAcknowledgment {
    pub(in crate::physical_runtime) fn from_completed(
        basis: PhysicalMutationAcknowledgmentBasis,
    ) -> Self {
        Self {
            mutation: basis.mutation,
            idempotency: basis.idempotency,
            fingerprint: basis.fingerprint,
            group_binding: basis.group_binding,
            policy: basis.policy,
            breadth: basis.breadth,
            records: basis.records,
            observation: basis.observation,
        }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.mutation
    }

    pub const fn idempotency_identity(&self) -> PhysicalMutationIdempotencyKeyIdentity {
        self.idempotency
    }

    pub const fn request_fingerprint(&self) -> PhysicalMutationRequestFingerprint {
        self.fingerprint
    }

    pub const fn group_binding(&self) -> PhysicalDurabilityGroupMemberBinding {
        self.group_binding
    }

    pub const fn durability_policy_identity(&self) -> PhysicalDurabilityPolicyIdentity {
        self.policy
    }

    pub const fn completed_breadth(&self) -> PhysicalMutationCompletedBreadth {
        self.breadth
    }

    pub fn persisted_records(&self) -> &[PersistedRecordIdentity] {
        &self.records
    }

    pub fn record_ids(
        &self,
    ) -> impl ExactSizeIterator<Item = crate::physical_runtime::PhysicalRecordId> + '_ {
        self.records
            .iter()
            .copied()
            .map(crate::physical_runtime::PhysicalRecordId::from_persisted)
    }

    pub const fn observation(&self) -> RecordAppendObservation {
        self.observation
    }

    pub fn executed_boundary_evidence(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationExecutedBoundaryEvidence {
        crate::physical_runtime::PhysicalMutationExecutedBoundaryEvidence::from_acknowledgment(self)
    }

    pub fn performance_evidence(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationPerformanceEvidence {
        crate::physical_runtime::PhysicalMutationPerformanceEvidence::from_acknowledgment(self)
    }
}
