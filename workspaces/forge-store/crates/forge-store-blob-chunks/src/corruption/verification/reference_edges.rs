use forge_store_contracts::StableDigest;

use crate::{
    BlobChunkIdentity, BlobChunkOrdinal, BlobChunkRegisteredDedupeReference,
    BlobCorruptionCounterSnapshot, BlobCorruptionDenial, BlobGeneration, BlobObjectId,
    BlobReachabilityStaging, BlobReachabilityStagingIdentity, BlobStreamingContentFrontier,
    ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCorruptionReferenceEdge {
    localized: BlobCorruptionReferenceGenerationBasis,
    affected: BlobCorruptionReferenceGenerationBasis,
    shared_chunk_identities: Option<BlobCorruptionSharedChunkIdentities>,
    edge_id: StableDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCorruptionReferenceEdges {
    edges: Vec<BlobCorruptionReferenceEdge>,
    edge_ids: Vec<StableDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlobCorruptionReferenceGenerationBasis {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    reachability_staging_basis: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlobCorruptionSharedChunkIdentities {
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
}

impl BlobCorruptionReferenceEdge {
    pub fn from_reachability_staging(staged: &BlobReachabilityStaging) -> Self {
        Self::from_reachability_staging_identity(staged.staging_identity())
    }

    pub fn from_reachability_staging_identity(staging: &BlobReachabilityStagingIdentity) -> Self {
        let basis = BlobCorruptionReferenceGenerationBasis::from_staging(staging);
        Self::from_parts(basis.clone(), basis, None, "single-reference")
    }

    pub fn from_admitted_shared_dedupe_reference(
        localized: &BlobReachabilityStagingIdentity,
        affected: &BlobReachabilityStagingIdentity,
        affected_frontier: &BlobStreamingContentFrontier,
        affected_ordinal: BlobChunkOrdinal,
        registered_reference: &BlobChunkRegisteredDedupeReference,
    ) -> Result<Self, BlobCorruptionDenial> {
        if affected.chunk_tree_root() != affected_frontier.chunk_tree_root()
            || affected.logical_content_digest() != affected_frontier.logical_content_digest()
            || localized.security_metadata() != registered_reference.security_metadata()
            || affected.security_metadata() != registered_reference.security_metadata()
        {
            return Err(BlobCorruptionDenial::AffectedReferenceEdgeMismatch {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        }
        let Some(affected_leaf) = affected_frontier
            .proof_frontier()
            .ordered_leaves()
            .iter()
            .find(|leaf| leaf.ordinal() == affected_ordinal)
        else {
            return Err(BlobCorruptionDenial::AffectedReferenceEdgeMismatch {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        };
        if !registered_reference.contains_chunk_identity(affected_leaf.identity())
            || registered_reference.security_metadata() != affected_leaf.security_metadata()
        {
            return Err(BlobCorruptionDenial::AffectedReferenceEdgeMismatch {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        }
        let localized = BlobCorruptionReferenceGenerationBasis::from_staging(localized);
        let affected = BlobCorruptionReferenceGenerationBasis::from_staging(affected);
        if localized == affected {
            return Err(BlobCorruptionDenial::DuplicateAffectedReferenceEdge {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        }
        let shared_chunk_identities = Some(BlobCorruptionSharedChunkIdentities {
            existing_identity: registered_reference.shared_identity().clone(),
            candidate_identity: registered_reference.candidate_identity().clone(),
        });
        Ok(Self::from_parts(
            localized,
            affected,
            shared_chunk_identities,
            registered_reference.content_digest().as_str(),
        ))
    }

    fn from_parts(
        localized: BlobCorruptionReferenceGenerationBasis,
        affected: BlobCorruptionReferenceGenerationBasis,
        shared_chunk_identities: Option<BlobCorruptionSharedChunkIdentities>,
        edge_authority_basis: &str,
    ) -> Self {
        let edge_id = StableDigest::new(format!(
            "s7-corruption-edge:v4:affected-reference:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            localized.object_id.digest().as_str(),
            localized.generation.sequence(),
            localized.chunk_tree_root.digest().as_str(),
            localized.logical_content_digest.digest().as_str(),
            localized.reachability_staging_basis,
            affected.object_id.digest().as_str(),
            affected.generation.sequence(),
            affected.chunk_tree_root.digest().as_str(),
            affected.logical_content_digest.digest().as_str(),
            affected.reachability_staging_basis,
            edge_authority_basis,
        ))
        .expect("corruption reference edge basis is nonempty");
        Self {
            localized,
            affected,
            shared_chunk_identities,
            edge_id,
        }
    }

    fn matches_localized_generation(
        &self,
        object_id: &BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: &ChunkTreeRoot,
        logical_content_digest: &LogicalContentDigest,
    ) -> bool {
        self.localized.matches_generation(
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
        )
    }

    fn matches_corrupted_chunk(&self, chunk_identity: &BlobChunkIdentity) -> bool {
        self.shared_chunk_identities
            .as_ref()
            .is_none_or(|identities| identities.matches(chunk_identity))
    }
}

impl BlobCorruptionReferenceEdges {
    pub fn from_reachability_staging(
        staged: &BlobReachabilityStaging,
    ) -> Result<Self, BlobCorruptionDenial> {
        Self::from_admitted_edges(&[BlobCorruptionReferenceEdge::from_reachability_staging(
            staged,
        )])
    }

    pub fn from_reachability_staging_identity(
        staging: &BlobReachabilityStagingIdentity,
    ) -> Result<Self, BlobCorruptionDenial> {
        Self::from_admitted_edges(&[
            BlobCorruptionReferenceEdge::from_reachability_staging_identity(staging),
        ])
    }

    pub fn from_reachability_staging_identities(
        staged: &[&BlobReachabilityStagingIdentity],
    ) -> Result<Self, BlobCorruptionDenial> {
        let edges: Vec<_> = staged
            .iter()
            .map(|staging| BlobCorruptionReferenceEdge::from_reachability_staging_identity(staging))
            .collect();
        Self::from_admitted_edges(&edges)
    }

    pub fn from_admitted_edges(
        edges: &[BlobCorruptionReferenceEdge],
    ) -> Result<Self, BlobCorruptionDenial> {
        if edges.is_empty() {
            return Err(BlobCorruptionDenial::EmptyAffectedReferenceEdges {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        }
        let mut edge_ids = Vec::with_capacity(edges.len());
        for edge in edges {
            if edge_ids.iter().any(|existing| existing == &edge.edge_id) {
                return Err(BlobCorruptionDenial::DuplicateAffectedReferenceEdge {
                    counters: BlobCorruptionCounterSnapshot::start().record_denial(),
                });
            }
            edge_ids.push(edge.edge_id.clone());
        }
        Ok(Self {
            edges: edges.to_vec(),
            edge_ids,
        })
    }

    pub fn edge_ids(&self) -> &[StableDigest] {
        &self.edge_ids
    }

    pub fn edge_count(&self) -> u64 {
        self.edge_ids.len() as u64
    }

    pub(crate) fn validated_edge_count_for_generation(
        &self,
        object_id: &BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: &ChunkTreeRoot,
        logical_content_digest: &LogicalContentDigest,
    ) -> Result<u64, BlobCorruptionDenial> {
        if self.edges.is_empty() {
            return Err(BlobCorruptionDenial::EmptyAffectedReferenceEdges {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        }
        for edge in &self.edges {
            if !edge.matches_localized_generation(
                object_id,
                generation,
                chunk_tree_root,
                logical_content_digest,
            ) {
                return Err(BlobCorruptionDenial::AffectedReferenceEdgeMismatch {
                    counters: BlobCorruptionCounterSnapshot::start().record_denial(),
                });
            }
        }
        Ok(self.edge_count())
    }

    pub(crate) fn validated_edge_count_for_corrupt_chunk(
        &self,
        object_id: &BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: &ChunkTreeRoot,
        logical_content_digest: &LogicalContentDigest,
        chunk_identity: &BlobChunkIdentity,
    ) -> Result<u64, BlobCorruptionDenial> {
        let edge_count = self.validated_edge_count_for_generation(
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
        )?;
        for edge in &self.edges {
            if !edge.matches_corrupted_chunk(chunk_identity) {
                return Err(BlobCorruptionDenial::AffectedReferenceEdgeMismatch {
                    counters: BlobCorruptionCounterSnapshot::start().record_denial(),
                });
            }
        }
        Ok(edge_count)
    }
}

impl BlobCorruptionReferenceGenerationBasis {
    fn from_staging(staging: &BlobReachabilityStagingIdentity) -> Self {
        Self {
            object_id: staging.object_id().clone(),
            generation: staging.generation(),
            chunk_tree_root: staging.chunk_tree_root().clone(),
            logical_content_digest: staging.logical_content_digest().clone(),
            reachability_staging_basis: staging.publication_record_digest(),
        }
    }

    fn matches_generation(
        &self,
        object_id: &BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: &ChunkTreeRoot,
        logical_content_digest: &LogicalContentDigest,
    ) -> bool {
        &self.object_id == object_id
            && self.generation == generation
            && &self.chunk_tree_root == chunk_tree_root
            && &self.logical_content_digest == logical_content_digest
    }
}

impl BlobCorruptionSharedChunkIdentities {
    fn matches(&self, chunk_identity: &BlobChunkIdentity) -> bool {
        &self.existing_identity == chunk_identity || &self.candidate_identity == chunk_identity
    }
}
