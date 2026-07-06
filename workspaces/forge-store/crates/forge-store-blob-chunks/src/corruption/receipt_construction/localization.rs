use crate::{
    BlobChunkOrdinal, BlobCorruptionCounterSnapshot, BlobCorruptionReferenceEdges, BlobDamageCase,
    BlobGeneration, BlobObjectId, StoredChunkDigest,
};
use crate::corruption::types::{
    BlobCorruptionDetectionSource, BlobCorruptionPlacementClass,
    BlobCorruptionReferenceSharingScope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCorruptedChunkLocalization {
    damage_case: BlobDamageCase,
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

pub(crate) fn construct_localization_receipt(
    damage_case: BlobDamageCase,
    source: BlobCorruptionDetectionSource,
    object_id: BlobObjectId,
    generation: BlobGeneration,
    ordinal: BlobChunkOrdinal,
    stored_digest: StoredChunkDigest,
    placement_class: BlobCorruptionPlacementClass,
    sharing_scope: BlobCorruptionReferenceSharingScope,
    reference_edges: BlobCorruptionReferenceEdges,
    edge_count: u64,
) -> BlobCorruptedChunkLocalization {
    BlobCorruptedChunkLocalization {
        damage_case,
        source,
        object_id,
        generation,
        ordinal,
        stored_digest,
        placement_class,
        sharing_scope,
        reference_edges,
        counters: BlobCorruptionCounterSnapshot::start()
            .record_localization(source, edge_count)
            .record_damage_case(damage_case),
    }
}

impl BlobCorruptedChunkLocalization {
    pub fn from_read_corruption(
        visible_generation: crate::BlobVisibleGeneration,
        frontier: crate::BlobStreamingContentFrontier,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
        reference_edges: BlobCorruptionReferenceEdges,
    ) -> Result<Self, crate::BlobCorruptionDenial> {
        crate::corruption::transitions::from_read_corruption(
            visible_generation,
            frontier,
            ordinal,
            placement_class,
            reference_edges,
        )
    }

    pub fn from_detected_source(
        source: BlobCorruptionDetectionSource,
        visible_generation: crate::BlobVisibleGeneration,
        frontier: crate::BlobStreamingContentFrontier,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
        reference_edges: BlobCorruptionReferenceEdges,
    ) -> Result<Self, crate::BlobCorruptionDenial> {
        crate::corruption::transitions::from_detected_source(
            source,
            visible_generation,
            frontier,
            ordinal,
            placement_class,
            reference_edges,
        )
    }

    pub fn from_streaming_read_request(
        request: &crate::BlobStreamingReadRequest,
        ordinal: BlobChunkOrdinal,
        placement_class: BlobCorruptionPlacementClass,
        damage_case: BlobDamageCase,
    ) -> Result<Self, crate::BlobCorruptionDenial> {
        crate::corruption::transitions::from_streaming_read_request(
            request,
            ordinal,
            placement_class,
            damage_case,
        )
    }

    pub const fn damage_case(&self) -> BlobDamageCase {
        self.damage_case
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