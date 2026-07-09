use worth_store_contracts::StableDigest;

mod identity;

use self::identity::{dedupe_edge_digest, edge_digest, edge_digest_from_parts};
use crate::{
    BlobChunkProofLeaf, BlobChunkRegisteredDedupeReference, BlobChunkSecurityMetadataWitness,
    BlobGeneration, BlobGenerationPublished, BlobLifecycleDeclaration, BlobObjectId,
    BlobPublicationIntent, BlobReachabilityCounterSnapshot, BlobReachabilityDenial, ChunkTreeRoot,
    LogicalContentDigest, ScopedBlobChunk, StoredChunkDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobReachabilityEdgeKind {
    PrimaryBlobReference,
    DerivedBlobReference,
    GenerationHoldReference,
    TimeWindowHoldReference,
    ResumeSessionReference,
    CheckpointHoldReference,
    BackupHoldReference,
    ExportHoldReference,
    TenantCustodyHoldReference,
    ExternalConsumerHoldReference,
    ReplicationCapsuleReference,
    ReadPlanHoldReference,
    QuarantineHoldReference,
    PlacementMoveReference,
    DedupeSharedReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReachabilityEdge {
    identity: StableDigest,
    kind: BlobReachabilityEdgeKind,
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    chunk_identity: crate::BlobChunkIdentity,
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    dedupe_reference_identity: Option<StableDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobReachabilityAuthorityKey {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobReachabilityEdge {
    pub fn primary_blob_reference(
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        Self::from_published_leaf(
            BlobReachabilityEdgeKind::PrimaryBlobReference,
            published,
            leaf,
        )
    }

    pub(crate) fn primary_lifecycle_reference(
        declaration: &BlobLifecycleDeclaration,
        scoped_chunk: ScopedBlobChunk,
    ) -> Result<Self, BlobReachabilityDenial> {
        let counters = BlobReachabilityCounterSnapshot::start();
        if declaration.stored_chunk_digest() != scoped_chunk.stored_digest()
            || declaration.security_metadata() != scoped_chunk.security_metadata()
        {
            return Err(BlobReachabilityDenial::WrongBlobAuthority {
                counters: counters.record_wrong_authority_denial(),
            });
        }
        let object_id = declaration.object_id().clone();
        let generation = declaration.generation();
        let chunk_tree_root = declaration.chunk_tree_root().clone();
        let logical_content_digest = declaration.logical_content_digest().clone();
        let chunk_identity = scoped_chunk.identity().clone();
        let stored_digest = scoped_chunk.stored_digest().clone();
        let security_metadata = declaration.security_metadata();
        Ok(Self {
            identity: edge_digest_from_parts(
                BlobReachabilityEdgeKind::PrimaryBlobReference,
                &object_id,
                generation,
                &chunk_tree_root,
                &logical_content_digest,
                chunk_identity.chunk_digest(),
                1,
            ),
            kind: BlobReachabilityEdgeKind::PrimaryBlobReference,
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            chunk_identity,
            stored_digest,
            security_metadata,
            dedupe_reference_identity: None,
        })
    }

    pub(crate) fn primary_lifecycle_multichunk_reference(
        declaration: &BlobLifecycleDeclaration,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        let counters = BlobReachabilityCounterSnapshot::start();
        if declaration.security_metadata() != leaf.security_metadata() {
            return Err(BlobReachabilityDenial::WrongBlobAuthority {
                counters: counters.record_wrong_authority_denial(),
            });
        }
        let object_id = declaration.object_id().clone();
        let generation = declaration.generation();
        let chunk_tree_root = declaration.chunk_tree_root().clone();
        let logical_content_digest = declaration.logical_content_digest().clone();
        let chunk_identity = leaf.identity().clone();
        let stored_digest = leaf.stored_digest().clone();
        let security_metadata = declaration.security_metadata();
        Ok(Self {
            identity: edge_digest_from_parts(
                BlobReachabilityEdgeKind::PrimaryBlobReference,
                &object_id,
                generation,
                &chunk_tree_root,
                &logical_content_digest,
                chunk_identity.chunk_digest(),
                leaf.ordinal().get(),
            ),
            kind: BlobReachabilityEdgeKind::PrimaryBlobReference,
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            chunk_identity,
            stored_digest,
            security_metadata,
            dedupe_reference_identity: None,
        })
    }

    pub fn derived_blob_reference(
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        Self::from_published_leaf(
            BlobReachabilityEdgeKind::DerivedBlobReference,
            published,
            leaf,
        )
    }

    pub fn resume_session_reference(
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        Self::from_published_leaf(
            BlobReachabilityEdgeKind::ResumeSessionReference,
            published,
            leaf,
        )
    }

    pub fn placement_move_reference(
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        Self::from_published_leaf(
            BlobReachabilityEdgeKind::PlacementMoveReference,
            published,
            leaf,
        )
    }

    pub(crate) fn dedupe_shared_reference(
        reference: &BlobChunkRegisteredDedupeReference,
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        let counters = BlobReachabilityCounterSnapshot::start();
        if !reference.contains_chunk_identity(leaf.identity())
            || reference.security_metadata() != leaf.security_metadata()
            || reference.security_metadata() != published.security_metadata()
        {
            return Err(BlobReachabilityDenial::DedupeReferenceMismatch {
                counters: counters.record_wrong_authority_denial(),
            });
        }
        let object_id = published.object_id().clone();
        let generation = published.generation();
        let chunk_tree_root = published.chunk_tree_root().clone();
        let logical_content_digest = published.logical_content_digest().clone();
        let chunk_identity = leaf.identity().clone();
        let stored_digest = leaf.stored_digest().clone();
        let security_metadata = leaf.security_metadata();
        Ok(Self {
            identity: dedupe_edge_digest(
                &object_id,
                generation,
                &chunk_tree_root,
                &logical_content_digest,
                leaf,
                reference.reference_identity(),
            ),
            kind: BlobReachabilityEdgeKind::DedupeSharedReference,
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            chunk_identity,
            stored_digest,
            security_metadata,
            dedupe_reference_identity: Some(reference.reference_identity().clone()),
        })
    }

    fn from_published_leaf(
        kind: BlobReachabilityEdgeKind,
        published: &BlobGenerationPublished,
        leaf: &BlobChunkProofLeaf,
    ) -> Result<Self, BlobReachabilityDenial> {
        let counters = BlobReachabilityCounterSnapshot::start();
        if published.security_metadata() != leaf.security_metadata() {
            return Err(BlobReachabilityDenial::WrongBlobAuthority {
                counters: counters.record_wrong_authority_denial(),
            });
        }
        if published.staging_identity().chunk_tree_root() != published.chunk_tree_root()
            || published.staging_identity().generation() != published.generation()
        {
            return Err(BlobReachabilityDenial::StaleGenerationEdge {
                counters: counters.record_stale_reference_denial(),
            });
        }
        let object_id = published.object_id().clone();
        let generation = published.generation();
        let chunk_tree_root = published.chunk_tree_root().clone();
        let logical_content_digest = published.logical_content_digest().clone();
        let chunk_identity = leaf.identity().clone();
        let stored_digest = leaf.stored_digest().clone();
        let security_metadata = leaf.security_metadata();
        Ok(Self {
            identity: edge_digest(
                kind,
                &object_id,
                generation,
                &chunk_tree_root,
                &logical_content_digest,
                leaf,
            ),
            kind,
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            chunk_identity,
            stored_digest,
            security_metadata,
            dedupe_reference_identity: None,
        })
    }

    pub const fn identity(&self) -> &StableDigest {
        &self.identity
    }

    pub const fn kind(&self) -> BlobReachabilityEdgeKind {
        self.kind
    }

    pub const fn chunk_identity(&self) -> &crate::BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub(crate) fn authority_key(&self) -> BlobReachabilityAuthorityKey {
        BlobReachabilityAuthorityKey {
            object_id: self.object_id.clone(),
            generation: self.generation,
            chunk_tree_root: self.chunk_tree_root.clone(),
            logical_content_digest: self.logical_content_digest.clone(),
            security_metadata: self.security_metadata,
        }
    }

    pub(crate) fn is_dedupe(&self) -> bool {
        self.kind == BlobReachabilityEdgeKind::DedupeSharedReference
    }

    pub(crate) fn dedupe_reference_identity(&self) -> Option<&StableDigest> {
        self.dedupe_reference_identity.as_ref()
    }
}

impl BlobReachabilityAuthorityKey {
    pub(crate) fn from_declaration(declaration: &BlobLifecycleDeclaration) -> Self {
        Self {
            object_id: declaration.object_id().clone(),
            generation: declaration.generation(),
            chunk_tree_root: declaration.chunk_tree_root().clone(),
            logical_content_digest: declaration.logical_content_digest().clone(),
            security_metadata: declaration.security_metadata(),
        }
    }

    pub(crate) fn from_published(published: &BlobGenerationPublished) -> Self {
        Self {
            object_id: published.object_id().clone(),
            generation: published.generation(),
            chunk_tree_root: published.chunk_tree_root().clone(),
            logical_content_digest: published.logical_content_digest().clone(),
            security_metadata: published.security_metadata(),
        }
    }

    pub(crate) fn matches(&self, edge: &BlobReachabilityEdge) -> bool {
        self.object_id == edge.object_id
            && self.generation == edge.generation
            && self.chunk_tree_root == edge.chunk_tree_root
            && self.logical_content_digest == edge.logical_content_digest
            && self.security_metadata == edge.security_metadata
    }

    pub(crate) fn matches_declaration(&self, declaration: &BlobLifecycleDeclaration) -> bool {
        self == &Self::from_declaration(declaration)
    }

    pub(crate) fn matches_publication_intent(&self, intent: &BlobPublicationIntent) -> bool {
        self.object_id == *intent.object_id()
            && self.generation == intent.generation()
            && self.chunk_tree_root == *intent.chunk_tree_root()
            && self.logical_content_digest == *intent.logical_content_digest()
            && self.security_metadata == intent.security_metadata()
    }

    pub(crate) fn hold_identity(&self, local_basis: &str) -> StableDigest {
        StableDigest::new(format!(
            "s7.reach.hold:{}:{}:{}:{}",
            self.object_id.digest().as_str(),
            self.generation.sequence(),
            self.chunk_tree_root.digest().as_str(),
            local_basis
        ))
        .expect("reachability hold digest is nonempty")
    }
}
