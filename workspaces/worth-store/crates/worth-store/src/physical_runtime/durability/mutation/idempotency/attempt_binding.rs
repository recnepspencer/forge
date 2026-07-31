use super::super::{PhysicalMutationIdentity, PhysicalMutationRequestFingerprint};
use super::PhysicalMutationIdempotencyKey;
use crate::physical_runtime::{CanonicalRedoRecords, PhysicalWalMemberBasis};

#[derive(Debug)]
pub(in crate::physical_runtime) struct WalUnallocated;

#[derive(Debug)]
pub(in crate::physical_runtime) struct WalAllocated {
    member: PhysicalWalMemberBasis,
    redo_digest: [u8; 32],
}

#[derive(Debug)]
pub(in crate::physical_runtime) struct PhysicalMutationAttemptBinding<State> {
    key: PhysicalMutationIdempotencyKey,
    fingerprint: PhysicalMutationRequestFingerprint,
    mutation: PhysicalMutationIdentity,
    state: State,
}

pub(in crate::physical_runtime) type UnallocatedPhysicalMutationAttemptBinding =
    PhysicalMutationAttemptBinding<WalUnallocated>;
pub(in crate::physical_runtime) type AllocatedPhysicalMutationAttemptBinding =
    PhysicalMutationAttemptBinding<WalAllocated>;

impl<State> PhysicalMutationAttemptBinding<State> {
    pub const fn key(&self) -> &PhysicalMutationIdempotencyKey {
        &self.key
    }

    pub const fn fingerprint(&self) -> PhysicalMutationRequestFingerprint {
        self.fingerprint
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.mutation
    }
}

impl PhysicalMutationAttemptBinding<WalUnallocated> {
    pub(super) const fn new(
        key: PhysicalMutationIdempotencyKey,
        fingerprint: PhysicalMutationRequestFingerprint,
        mutation: PhysicalMutationIdentity,
    ) -> Self {
        Self {
            key,
            fingerprint,
            mutation,
            state: WalUnallocated,
        }
    }

    pub(in crate::physical_runtime) fn allocate_wal(
        self,
        member: PhysicalWalMemberBasis,
        redo: &CanonicalRedoRecords,
    ) -> AllocatedPhysicalMutationAttemptBinding {
        debug_assert_eq!(self.mutation, member.mutation_identity());
        PhysicalMutationAttemptBinding {
            key: self.key,
            fingerprint: self.fingerprint,
            mutation: self.mutation,
            state: WalAllocated {
                member,
                redo_digest: redo.digest(),
            },
        }
    }
}

impl PhysicalMutationAttemptBinding<WalAllocated> {
    pub(in crate::physical_runtime) fn release_wal_allocation(
        self,
    ) -> UnallocatedPhysicalMutationAttemptBinding {
        PhysicalMutationAttemptBinding {
            key: self.key,
            fingerprint: self.fingerprint,
            mutation: self.mutation,
            state: WalUnallocated,
        }
    }

    pub const fn member(&self) -> PhysicalWalMemberBasis {
        self.state.member
    }

    pub const fn redo_digest(&self) -> [u8; 32] {
        self.state.redo_digest
    }

    pub(in crate::physical_runtime) fn encode_persisted(&self) -> Vec<u8> {
        let lease = self.key.lease();
        let mutation = self.mutation;
        let range = self.state.member.lsn_range();
        let mut bytes = Vec::with_capacity(224);
        write_field(&mut bytes, b"store.physical.mutation-attempt-binding.v1");
        write_field(&mut bytes, &self.key.identity().bytes());
        bytes.extend_from_slice(&lease.issuance_generation().get().to_le_bytes());
        bytes.extend_from_slice(&lease.expiry_generation().get().to_le_bytes());
        write_field(&mut bytes, &self.fingerprint.bytes());
        write_field(&mut bytes, &mutation.store_identity().bytes());
        bytes.extend_from_slice(&mutation.runtime_identity().get().to_le_bytes());
        bytes.extend_from_slice(&mutation.operation_identity().get().to_le_bytes());
        write_field(&mut bytes, &self.state.member.member_identity().bytes());
        bytes.extend_from_slice(&range.start().get().to_le_bytes());
        bytes.extend_from_slice(&range.end_exclusive().get().to_le_bytes());
        write_field(&mut bytes, &self.state.redo_digest);
        bytes
    }
}

fn write_field(target: &mut Vec<u8>, field: &[u8]) {
    target.extend_from_slice(&(field.len() as u64).to_le_bytes());
    target.extend_from_slice(field);
}
