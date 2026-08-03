use worth_store_physical_format::PersistedRecordIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRecoveryOperationFact {
    idempotency: crate::physical_runtime::PhysicalMutationIdempotencyKeyIdentity,
    lease: crate::physical_runtime::PhysicalMutationIdempotencyLease,
    fingerprint: crate::physical_runtime::PhysicalMutationRequestFingerprint,
    mutation: crate::physical_runtime::PhysicalMutationIdentity,
    attempt: PhysicalRecoveryAttemptBindingFact,
    fate: PhysicalRecoveryOperationFate,
    last_compacted: Option<crate::physical_runtime::PhysicalNamespaceDurableCheckpointGeneration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRecoveryAttemptBindingFact {
    Unsealed,
    GroupSealed(crate::physical_runtime::PhysicalDurabilityGroupMemberBinding),
    WalBound(PhysicalRecoveryWalAttemptBinding),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRecoveryWalAttemptBinding {
    group: crate::physical_runtime::PhysicalDurabilityGroupMemberBinding,
    member: crate::physical_runtime::PhysicalWalMemberBasis,
    redo_digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRecoveryOperationFate {
    Unresolved,
    Completed(PhysicalRecoveryCompletedMutationFact),
    ProvenNoEffect(crate::physical_runtime::ProvenNoEffectPhysicalMutation),
    Indeterminate(crate::physical_runtime::IndeterminatePhysicalMutation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRecoveryCompletedMutationFact {
    breadth: crate::physical_runtime::PhysicalMutationCompletedBreadth,
    records: Box<[PersistedRecordIdentity]>,
    observation: crate::physical_runtime::RecordAppendObservation,
}

impl PhysicalRecoveryOperationFact {
    pub(in crate::physical_runtime::durability) const fn new(
        idempotency: crate::physical_runtime::PhysicalMutationIdempotencyKeyIdentity,
        lease: crate::physical_runtime::PhysicalMutationIdempotencyLease,
        fingerprint: crate::physical_runtime::PhysicalMutationRequestFingerprint,
        mutation: crate::physical_runtime::PhysicalMutationIdentity,
        attempt: PhysicalRecoveryAttemptBindingFact,
        fate: PhysicalRecoveryOperationFate,
        last_compacted: Option<
            crate::physical_runtime::PhysicalNamespaceDurableCheckpointGeneration,
        >,
    ) -> Self {
        Self {
            idempotency,
            lease,
            fingerprint,
            mutation,
            attempt,
            fate,
            last_compacted,
        }
    }

    pub const fn idempotency_identity(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationIdempotencyKeyIdentity {
        self.idempotency
    }

    pub const fn lease(&self) -> crate::physical_runtime::PhysicalMutationIdempotencyLease {
        self.lease
    }

    pub const fn request_fingerprint(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationRequestFingerprint {
        self.fingerprint
    }

    pub const fn mutation_identity(&self) -> crate::physical_runtime::PhysicalMutationIdentity {
        self.mutation
    }

    pub const fn attempt(&self) -> &PhysicalRecoveryAttemptBindingFact {
        &self.attempt
    }

    pub const fn fate(&self) -> &PhysicalRecoveryOperationFate {
        &self.fate
    }

    pub const fn last_compacted_generation(
        &self,
    ) -> Option<crate::physical_runtime::PhysicalNamespaceDurableCheckpointGeneration> {
        self.last_compacted
    }
}

impl PhysicalRecoveryWalAttemptBinding {
    pub(in crate::physical_runtime::durability) const fn from_persisted(
        binding: &crate::physical_runtime::durability::PersistedPhysicalMutationAttemptBinding,
    ) -> Self {
        Self {
            group: binding.group(),
            member: binding.member(),
            redo_digest: binding.redo_digest(),
        }
    }

    pub const fn group(&self) -> crate::physical_runtime::PhysicalDurabilityGroupMemberBinding {
        self.group
    }

    pub const fn member(&self) -> crate::physical_runtime::PhysicalWalMemberBasis {
        self.member
    }

    pub const fn redo_digest(&self) -> [u8; 32] {
        self.redo_digest
    }
}

impl PhysicalRecoveryCompletedMutationFact {
    pub(in crate::physical_runtime::durability) fn from_completed(
        fact: &crate::physical_runtime::durability::CompletedPhysicalMutationFact,
    ) -> Self {
        Self {
            breadth: fact.breadth(),
            records: fact.persisted_records().to_vec().into_boxed_slice(),
            observation: fact.observation(),
        }
    }

    pub const fn completed_breadth(
        &self,
    ) -> crate::physical_runtime::PhysicalMutationCompletedBreadth {
        self.breadth
    }

    pub fn persisted_records(&self) -> &[PersistedRecordIdentity] {
        &self.records
    }

    pub const fn observation(&self) -> crate::physical_runtime::RecordAppendObservation {
        self.observation
    }
}
