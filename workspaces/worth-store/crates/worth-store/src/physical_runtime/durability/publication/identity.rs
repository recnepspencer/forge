use sha2::{Digest, Sha256};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordArtifactFile};

use crate::physical_runtime::{
    PhysicalDurabilityGroupBasis, PhysicalDurabilityGroupIdentity,
    PhysicalDurabilityGroupMemberBinding, PhysicalDurabilityPolicyIdentity,
    PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdentity, PhysicalWalMemberIdentity,
    RuntimeIdentity,
};

/// Exact identity shared by every effect in one current-root transition.
///
/// Construction remains inside the durability owner. Work declarations may
/// carry this identity but cannot mint or alter it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct PhysicalRootPublicationIdentity {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    policy: PhysicalDurabilityPolicyIdentity,
    group: PhysicalDurabilityGroupIdentity,
    membership: [u8; 32],
    member_count: u32,
    source_generation: u64,
    candidate_generation: u64,
    catalog_candidate: RecordArtifactFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRootPublicationMemberIdentity {
    mutation: PhysicalMutationIdentity,
    wal_member: PhysicalWalMemberIdentity,
    idempotency: PhysicalMutationIdempotencyKeyIdentity,
    binding: PhysicalDurabilityGroupMemberBinding,
}

impl PhysicalRootPublicationIdentity {
    pub(in crate::physical_runtime) fn from_settled_group(
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        policy: PhysicalDurabilityPolicyIdentity,
        group: PhysicalDurabilityGroupBasis,
        source_generation: u64,
        candidate_publication: u64,
    ) -> Option<Self> {
        let candidate_generation = source_generation.checked_add(1)?;
        (source_generation != 0 && candidate_publication != 0).then_some(Self {
            store,
            runtime,
            policy,
            group: group.identity(),
            membership: group.membership_digest(),
            member_count: group.member_count().get(),
            source_generation,
            candidate_generation,
            catalog_candidate: RecordArtifactFile::CatalogCandidate {
                publication: candidate_publication,
            },
        })
    }

    pub(in crate::physical_runtime) const fn group(self) -> PhysicalDurabilityGroupIdentity {
        self.group
    }

    pub(in crate::physical_runtime) const fn membership(self) -> [u8; 32] {
        self.membership
    }

    pub(in crate::physical_runtime) const fn member_count(self) -> u32 {
        self.member_count
    }

    pub(in crate::physical_runtime) const fn source_generation(self) -> u64 {
        self.source_generation
    }

    pub(in crate::physical_runtime) const fn candidate_generation(self) -> u64 {
        self.candidate_generation
    }

    pub(in crate::physical_runtime) const fn catalog_candidate(self) -> RecordArtifactFile {
        self.catalog_candidate
    }

    pub(in crate::physical_runtime) fn stable_digest(self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(b"worth-store.root-publication-identity.v1");
        digest.update(self.store.bytes());
        digest.update(self.runtime.get().to_le_bytes());
        digest.update(self.policy.bytes());
        digest.update(self.group.bytes());
        digest.update(self.membership);
        digest.update(self.member_count.to_le_bytes());
        digest.update(self.source_generation.to_le_bytes());
        digest.update(self.candidate_generation.to_le_bytes());
        let RecordArtifactFile::CatalogCandidate { publication } = self.catalog_candidate else {
            unreachable!("root identity construction fixes the catalog candidate family")
        };
        digest.update(publication.to_le_bytes());
        digest.finalize().into()
    }
}

impl PhysicalRootPublicationMemberIdentity {
    pub(in crate::physical_runtime) const fn new(
        mutation: PhysicalMutationIdentity,
        wal_member: PhysicalWalMemberIdentity,
        idempotency: PhysicalMutationIdempotencyKeyIdentity,
        binding: PhysicalDurabilityGroupMemberBinding,
    ) -> Self {
        Self {
            mutation,
            wal_member,
            idempotency,
            binding,
        }
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.mutation
    }

    pub const fn wal_member_identity(self) -> PhysicalWalMemberIdentity {
        self.wal_member
    }

    pub const fn idempotency_identity(self) -> PhysicalMutationIdempotencyKeyIdentity {
        self.idempotency
    }

    pub const fn group_binding(self) -> PhysicalDurabilityGroupMemberBinding {
        self.binding
    }
}
