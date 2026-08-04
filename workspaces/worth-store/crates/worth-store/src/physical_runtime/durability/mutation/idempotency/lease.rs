use crate::physical_runtime::{IdempotencyRetentionGenerations, PhysicalDurabilityPolicyIdentity};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalNamespaceDurableCheckpointGeneration(u64);

impl PhysicalNamespaceDurableCheckpointGeneration {
    pub const INITIAL: Self = Self(0);

    pub const fn get(self) -> u64 {
        self.0
    }

    pub(super) const fn checked_successor(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(generation) => Some(Self(generation)),
            None => None,
        }
    }

    pub(super) const fn from_reopened(generation: u64) -> Self {
        Self(generation)
    }

    #[cfg(test)]
    pub(super) const fn from_namespace_durable_checkpoint(generation: u64) -> Self {
        Self(generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMutationIdempotencyLease {
    store: StableStoreIdentity,
    policy: PhysicalDurabilityPolicyIdentity,
    issuance: PhysicalNamespaceDurableCheckpointGeneration,
    expiry: PhysicalNamespaceDurableCheckpointGeneration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PhysicalMutationLeaseIssuanceFailure {
    GenerationExhausted,
}

impl PhysicalMutationIdempotencyLease {
    pub(super) fn from_reopened(
        store: StableStoreIdentity,
        policy: PhysicalDurabilityPolicyIdentity,
        issuance: u64,
        expiry: u64,
        retention: IdempotencyRetentionGenerations,
    ) -> Option<Self> {
        let expected_expiry = issuance.checked_add(retention.get().get())?;
        if expiry != expected_expiry {
            return None;
        }
        Some(Self {
            store,
            policy,
            issuance: PhysicalNamespaceDurableCheckpointGeneration(issuance),
            expiry: PhysicalNamespaceDurableCheckpointGeneration(expiry),
        })
    }

    pub(super) fn issue(
        store: StableStoreIdentity,
        policy: PhysicalDurabilityPolicyIdentity,
        issuance: PhysicalNamespaceDurableCheckpointGeneration,
        retention: IdempotencyRetentionGenerations,
    ) -> Result<Self, PhysicalMutationLeaseIssuanceFailure> {
        let expiry = issuance
            .get()
            .checked_add(retention.get().get())
            .ok_or(PhysicalMutationLeaseIssuanceFailure::GenerationExhausted)?;
        Ok(Self {
            store,
            policy,
            issuance,
            expiry: PhysicalNamespaceDurableCheckpointGeneration(expiry),
        })
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn policy_identity(self) -> PhysicalDurabilityPolicyIdentity {
        self.policy
    }

    pub const fn issuance_generation(self) -> PhysicalNamespaceDurableCheckpointGeneration {
        self.issuance
    }

    pub const fn expiry_generation(self) -> PhysicalNamespaceDurableCheckpointGeneration {
        self.expiry
    }

    pub const fn is_expired_at(
        self,
        current: PhysicalNamespaceDurableCheckpointGeneration,
    ) -> bool {
        current.get() >= self.expiry.get()
    }
}
