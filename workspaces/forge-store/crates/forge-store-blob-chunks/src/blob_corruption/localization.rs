use crate::{
    BlobChunkOrdinal, BlobCorruptionCounterSnapshot, BlobCorruptionDenial,
    BlobCorruptionReferenceEdges, BlobGeneration, BlobObjectId, BlobStreamingContentFrontier,
    BlobStreamingReadRequest, BlobVisibleGeneration, StoredChunkDigest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionDetectionSource {
    VerifiedRead,
    Scrub,
    ColdFetch,
    ImportReadmission,
    CapsuleMaterialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionPlacementClass {
    LocalPhysical,
    ExternalCold,
    ImportStaging,
    CapsuleMaterialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCorruptionReferenceSharingScope {
    SingleReference,
    SharedReferenceEdges,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCorruptedChunkLocalization {
    source: BlobCorruptionDetectionSource,
    object_id: BlobObjectId,
    generation: BlobGeneration,
    ordinal: BlobChunkOrdinal,
    stored_digest: StoredChunkDigest,
    placement_class: BlobCorruptionPlacementClass,
    sharing_scope: BlobCorruptionReferenceSharingScope,
    reference_edges: BlobCorruptionReferenceEdges,
    counters: BlobCorruptionCounterSnapshot,
}

impl BlobCorruptedChunkLocalization {
    pub fn from_read_corruption(
        visible_generation: BlobVisibleGeneration,
        frontier: BlobStreamingContentFrontier,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
        reference_edges: BlobCorruptionReferenceEdges,
    ) -> Result<Self, BlobCorruptionDenial> {
        Self::from_detected_source(
            BlobCorruptionDetectionSource::VerifiedRead,
            visible_generation,
            frontier,
            ordinal,
            placement_class,
            reference_edges,
        )
    }

    pub fn from_detected_source(
        source: BlobCorruptionDetectionSource,
        visible_generation: BlobVisibleGeneration,
        frontier: BlobStreamingContentFrontier,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
        reference_edges: BlobCorruptionReferenceEdges,
    ) -> Result<Self, BlobCorruptionDenial> {
        if visible_generation.chunk_tree_root() != frontier.chunk_tree_root()
            || visible_generation.logical_content_digest() != frontier.logical_content_digest()
        {
            return Err(BlobCorruptionDenial::GenerationFrontierMismatch {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        }
        Self::from_bound_parts(
            source,
            visible_generation.object_id().clone(),
            visible_generation.generation(),
            &frontier,
            ordinal,
            placement_class,
            reference_edges,
        )
    }

    pub fn from_streaming_read_request(
        request: &BlobStreamingReadRequest,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
    ) -> Result<Self, BlobCorruptionDenial> {
        Self::from_bound_parts(
            BlobCorruptionDetectionSource::VerifiedRead,
            request.object_id().clone(),
            request.generation(),
            request.frontier(),
            ordinal,
            placement_class,
            request.corruption_reference_edges().clone(),
        )
    }

    fn from_bound_parts(
        source: BlobCorruptionDetectionSource,
        object_id: BlobObjectId,
        generation: BlobGeneration,
        frontier: &BlobStreamingContentFrontier,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
        reference_edges: BlobCorruptionReferenceEdges,
    ) -> Result<Self, BlobCorruptionDenial> {
        let Some(leaf) = frontier
            .proof_frontier()
            .ordered_leaves()
            .iter()
            .find(|leaf| leaf.ordinal() == ordinal)
        else {
            return Err(BlobCorruptionDenial::CorruptOrdinalNotInPublishedFrontier {
                ordinal,
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            });
        };
        let edge_count = reference_edges.validated_edge_count_for_corrupt_chunk(
            &object_id,
            generation,
            frontier.chunk_tree_root(),
            frontier.logical_content_digest(),
            leaf.identity(),
        )?;
        let sharing_scope = if edge_count == 1 {
            BlobCorruptionReferenceSharingScope::SingleReference
        } else {
            BlobCorruptionReferenceSharingScope::SharedReferenceEdges
        };
        Ok(Self {
            source,
            object_id,
            generation,
            ordinal,
            stored_digest: leaf.stored_digest().clone(),
            placement_class,
            sharing_scope,
            reference_edges,
            counters: BlobCorruptionCounterSnapshot::start()
                .record_localization(source, edge_count),
        })
    }

    pub const fn source(&self) -> BlobCorruptionDetectionSource {
        self.source
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn ordinal(&self) -> BlobChunkOrdinal {
        self.ordinal
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn placement_class(&self) -> BlobCorruptionPlacementClass {
        self.placement_class
    }

    pub const fn sharing_scope(&self) -> BlobCorruptionReferenceSharingScope {
        self.sharing_scope
    }

    pub const fn reference_edges(&self) -> &BlobCorruptionReferenceEdges {
        &self.reference_edges
    }

    pub const fn counters(&self) -> BlobCorruptionCounterSnapshot {
        self.counters
    }
}
