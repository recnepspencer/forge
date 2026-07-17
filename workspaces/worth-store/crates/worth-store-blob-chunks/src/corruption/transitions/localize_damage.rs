#[path = "localize_damage_steps.rs"]
mod localize_damage_steps;

use crate::corruption::classification::{
    classify_blob_damage_before_decode, BlobDamageEvidence, LocalizationEligibilityCase,
};
use crate::corruption::receipt_construction::{
    construct_localization_receipt, BlobLocalizationReceiptInput,
};
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

struct DamageLocalizationInput<'a> {
    source: BlobCorruptionDetectionSource,
    damage_case: BlobDamageCase,
    object_id: crate::BlobObjectId,
    generation: crate::BlobGeneration,
    frontier: &'a BlobStreamingContentFrontier,
    ordinal: BlobChunkOrdinal,
    placement_class: BlobCorruptionPlacementClass,
    reference_edges: BlobCorruptionReferenceEdges,
}

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
    localize_damage_from_bound_parts(DamageLocalizationInput {
        source,
        damage_case,
        object_id: visible_generation.object_id().clone(),
        generation: visible_generation.generation(),
        frontier: &frontier,
        ordinal,
        placement_class,
        reference_edges,
    })
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
    localize_damage_from_bound_parts(DamageLocalizationInput {
        source: BlobCorruptionDetectionSource::VerifiedRead,
        damage_case,
        object_id: request.object_id().clone(),
        generation: request.generation(),
        frontier: request.frontier(),
        ordinal,
        placement_class,
        reference_edges: request.corruption_reference_edges().clone(),
    })
}

fn localize_damage_from_bound_parts(
    input: DamageLocalizationInput<'_>,
) -> Result<BlobCorruptedChunkLocalization, BlobCorruptionDenial> {
    if let Err(eligibility) = verify_ordinal_localization_eligibility(input.frontier, input.ordinal)
    {
        return Err(assemble_localization_denial(eligibility, input.ordinal));
    }
    let leaf = resolve_corrupt_chunk_leaf(input.frontier, input.ordinal);
    let edge_count = validate_corrupt_chunk_reference_edges(
        &input.reference_edges,
        &input.object_id,
        input.generation,
        input.frontier,
        leaf,
    )?;
    let sharing_scope = classify_reference_sharing_scope(edge_count);
    Ok(construct_localization_receipt(
        BlobLocalizationReceiptInput {
            damage_case: input.damage_case,
            source: input.source,
            object_id: input.object_id,
            generation: input.generation,
            ordinal: input.ordinal,
            stored_digest: leaf.stored_digest().clone(),
            placement_class: input.placement_class,
            sharing_scope,
            reference_edges: input.reference_edges,
            edge_count,
        },
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
