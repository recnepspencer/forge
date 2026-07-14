#[path = "localize_damage_steps.rs"]
mod localize_damage_steps;

use crate::corruption::classification::{
    classify_blob_damage_before_decode, BlobDamageEvidence, LocalizationEligibilityCase,
};
use crate::corruption::receipt_construction::construct_localization_receipt;
use crate::corruption::types::{BlobCorruptionDetectionSource, BlobCorruptionPlacementClass};
use crate::corruption::verification::verify_generation_frontier_match;
use crate::{
    BlobChunkOrdinal, BlobCorruptedChunkLocalization, BlobCorruptionCounterSnapshot,
    BlobCorruptionDenial, BlobCorruptionReferenceEdges, BlobDamageCase,
    BlobStreamingContentFrontier, BlobStreamingReadRequest, BlobVisibleGeneration,
};

use localize_damage_steps::{
    classify_reference_sharing_scope, resolve_corrupt_chunk_leaf,
    validate_corrupt_chunk_reference_edges, verify_ordinal_localization_eligibility,
};

pub fn localize_detected_damage(
    source: BlobCorruptionDetectionSource,
    damage_case: BlobDamageCase,
    visible_generation: BlobVisibleGeneration,
    frontier: BlobStreamingContentFrontier,
    ordinal: BlobChunkOrdinal,
    placement_class: BlobCorruptionPlacementClass,
    reference_edges: BlobCorruptionReferenceEdges,
) -> Result<BlobCorruptedChunkLocalization, BlobCorruptionDenial> {
    verify_generation_frontier_match(&visible_generation, &frontier)?;
    localize_damage_from_bound_parts(
        source,
        damage_case,
        visible_generation.object_id().clone(),
        visible_generation.generation(),
        &frontier,
        ordinal,
        placement_class,
        reference_edges,
    )
}

pub fn from_read_corruption(
    visible_generation: BlobVisibleGeneration,
    frontier: BlobStreamingContentFrontier,
    ordinal: BlobChunkOrdinal,
    placement_class: BlobCorruptionPlacementClass,
    reference_edges: BlobCorruptionReferenceEdges,
) -> Result<BlobCorruptedChunkLocalization, BlobCorruptionDenial> {
    let damage_case = classify_blob_damage_before_decode(BlobDamageEvidence::DetectionContext {
        source: BlobCorruptionDetectionSource::VerifiedRead,
        placement: placement_class,
    });
    localize_detected_damage(
        BlobCorruptionDetectionSource::VerifiedRead,
        damage_case,
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
) -> Result<BlobCorruptedChunkLocalization, BlobCorruptionDenial> {
    let damage_case = classify_blob_damage_before_decode(BlobDamageEvidence::DetectionContext {
        source,
        placement: placement_class,
    });
    localize_detected_damage(
        source,
        damage_case,
        visible_generation,
        frontier,
        ordinal,
        placement_class,
        reference_edges,
    )
}

pub fn from_streaming_read_request(
    request: &BlobStreamingReadRequest,
    ordinal: BlobChunkOrdinal,
    placement_class: BlobCorruptionPlacementClass,
    damage_case: BlobDamageCase,
) -> Result<BlobCorruptedChunkLocalization, BlobCorruptionDenial> {
    localize_damage_from_bound_parts(
        BlobCorruptionDetectionSource::VerifiedRead,
        damage_case,
        request.object_id().clone(),
        request.generation(),
        request.frontier(),
        ordinal,
        placement_class,
        request.corruption_reference_edges().clone(),
    )
}

fn localize_damage_from_bound_parts(
    source: BlobCorruptionDetectionSource,
    damage_case: BlobDamageCase,
    object_id: crate::BlobObjectId,
    generation: crate::BlobGeneration,
    frontier: &BlobStreamingContentFrontier,
    ordinal: BlobChunkOrdinal,
    placement_class: BlobCorruptionPlacementClass,
    reference_edges: BlobCorruptionReferenceEdges,
) -> Result<BlobCorruptedChunkLocalization, BlobCorruptionDenial> {
    if let Err(eligibility) = verify_ordinal_localization_eligibility(frontier, ordinal) {
        return Err(assemble_localization_denial(eligibility, ordinal));
    }
    let leaf = resolve_corrupt_chunk_leaf(frontier, ordinal);
    let edge_count = validate_corrupt_chunk_reference_edges(
        &reference_edges,
        &object_id,
        generation,
        frontier,
        leaf,
    )?;
    let sharing_scope = classify_reference_sharing_scope(edge_count);
    Ok(construct_localization_receipt(
        damage_case,
        source,
        object_id,
        generation,
        ordinal,
        leaf.stored_digest().clone(),
        placement_class,
        sharing_scope,
        reference_edges,
        edge_count,
    ))
}

fn assemble_localization_denial(
    case: LocalizationEligibilityCase,
    ordinal: BlobChunkOrdinal,
) -> BlobCorruptionDenial {
    let damage_case = match case {
        LocalizationEligibilityCase::OrdinalNotInFrontier => {
            classify_blob_damage_before_decode(BlobDamageEvidence::OrdinalNotInFrontier)
        }
        LocalizationEligibilityCase::GenerationFrontierMismatch => {
            classify_blob_damage_before_decode(BlobDamageEvidence::GenerationFrontierMismatch)
        }
        LocalizationEligibilityCase::FrontierMatched => {
            unreachable!("matched frontier denies nothing")
        }
    };
    match case {
        LocalizationEligibilityCase::OrdinalNotInFrontier => {
            BlobCorruptionDenial::CorruptOrdinalNotInPublishedFrontier {
                damage_case,
                ordinal,
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            }
        }
        LocalizationEligibilityCase::GenerationFrontierMismatch => {
            BlobCorruptionDenial::GenerationFrontierMismatch {
                damage_case,
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            }
        }
        LocalizationEligibilityCase::FrontierMatched => {
            unreachable!("matched frontier denies nothing")
        }
    }
}
