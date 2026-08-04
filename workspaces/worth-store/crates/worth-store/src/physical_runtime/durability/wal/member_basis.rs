use sha2::{Digest, Sha256};
use worth_store_wal::WalLsnRange;

use crate::physical_runtime::PhysicalMutationIdentity;

const MEMBER_IDENTITY_DOMAIN: &[u8] = b"store.physical.wal-member-identity.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWalMemberIdentity([u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalMemberBasis {
    member: PhysicalWalMemberIdentity,
    mutation: PhysicalMutationIdentity,
    range: WalLsnRange,
}

impl PhysicalWalMemberIdentity {
    pub(in crate::physical_runtime) fn for_mutation(mutation: PhysicalMutationIdentity) -> Self {
        let mut digest = Sha256::new();
        digest.update(MEMBER_IDENTITY_DOMAIN);
        digest.update(mutation.store_identity().bytes());
        digest.update(mutation.runtime_identity().get().to_le_bytes());
        digest.update(mutation.operation_identity().get().to_le_bytes());
        Self(digest.finalize().into())
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

impl PhysicalWalMemberBasis {
    pub(in crate::physical_runtime) const fn new(
        member: PhysicalWalMemberIdentity,
        mutation: PhysicalMutationIdentity,
        range: WalLsnRange,
    ) -> Self {
        Self {
            member,
            mutation,
            range,
        }
    }

    pub const fn member_identity(self) -> PhysicalWalMemberIdentity {
        self.member
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.mutation
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.range
    }
}
