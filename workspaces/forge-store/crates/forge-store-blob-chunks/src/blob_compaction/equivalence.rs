use super::{BlobCompactionDenial, BlobCompactionRewritePlan};
use crate::{
    BlobAuthorityClassification, BlobChunkRootCanonicalBasis, BlobChunkRootPublication,
    BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId, BlobStreamingVerifiedRead,
    ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCompactionEquivalence {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    old_root: ChunkTreeRoot,
    new_root: ChunkTreeRoot,
    logical_digest: LogicalContentDigest,
    security: BlobChunkSecurityMetadataWitness,
    authority_class: BlobAuthorityClassification,
    uncompacted_canonical_basis: BlobChunkRootCanonicalBasis,
    canonical_basis: BlobChunkRootCanonicalBasis,
    reachable_chunks: u64,
    reference_edges: u64,
    verified_bytes: u64,
}

impl BlobCompactionEquivalence {
    pub fn from_rewritten_root_and_verified_read(
        plan: &BlobCompactionRewritePlan,
        rewritten: &BlobChunkRootPublication,
        read: &BlobStreamingVerifiedRead,
    ) -> Result<Self, BlobCompactionDenial> {
        let basis = plan.basis();
        if read.object_id() != basis.object_id()
            || read.generation() != basis.generation()
            || read.chunk_tree_root() != rewritten.chunk_tree_root()
            || read.logical_content_digest() != basis.logical_digest()
            || rewritten.logical_content_digest() != basis.logical_digest()
            || rewritten.canonical_basis().logical_content_digest() != basis.logical_digest()
            || rewritten.canonical_basis().total_bytes() != read.counters().bytes_read()
            || rewritten.canonical_basis().total_bytes() != plan.old_canonical_basis().total_bytes()
            || rewritten.canonical_basis().canonical_digest()
                != plan.old_canonical_basis().canonical_digest()
            || plan.reachability().security_metadata() != basis.security()
            || plan.placement().security_metadata() != basis.security()
            || plan.placement().stored_digest() != basis.stored_digest()
        {
            return Err(BlobCompactionDenial::EquivalenceBasisMismatch {
                counters: plan.counters().record_denial(),
            });
        }
        Ok(Self {
            object_id: basis.object_id().clone(),
            generation: basis.generation(),
            old_root: basis.old_root().clone(),
            new_root: rewritten.chunk_tree_root().clone(),
            logical_digest: basis.logical_digest().clone(),
            security: basis.security(),
            authority_class: basis.authority_class(),
            uncompacted_canonical_basis: plan.old_canonical_basis().clone(),
            canonical_basis: rewritten.canonical_basis().clone(),
            reachable_chunks: plan.reachability().reachable_chunks().len() as u64,
            reference_edges: plan.reachability().reference_edges().len() as u64,
            verified_bytes: rewritten.canonical_basis().total_bytes(),
        })
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn old_root(&self) -> &ChunkTreeRoot {
        &self.old_root
    }

    pub const fn new_root(&self) -> &ChunkTreeRoot {
        &self.new_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security
    }

    pub const fn authority_classification(&self) -> BlobAuthorityClassification {
        self.authority_class
    }

    pub const fn canonical_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.canonical_basis
    }

    pub const fn uncompacted_canonical_basis(&self) -> &BlobChunkRootCanonicalBasis {
        &self.uncompacted_canonical_basis
    }

    pub const fn reachable_chunks(&self) -> u64 {
        self.reachable_chunks
    }

    pub const fn reference_edges(&self) -> u64 {
        self.reference_edges
    }

    pub const fn verified_bytes(&self) -> u64 {
        self.verified_bytes
    }

    pub(crate) fn matches_plan_basis(&self, plan: &BlobCompactionRewritePlan) -> bool {
        let basis = plan.basis();
        self.object_id == *basis.object_id()
            && self.generation == basis.generation()
            && self.old_root == *basis.old_root()
            && self.logical_digest == *basis.logical_digest()
            && self.security == basis.security()
            && self.authority_class == basis.authority_class()
            && self.uncompacted_canonical_basis.canonical_digest()
                == plan.old_canonical_basis().canonical_digest()
            && self.reachable_chunks == plan.reachability().reachable_chunks().len() as u64
            && self.reference_edges == plan.reachability().reference_edges().len() as u64
    }
}
