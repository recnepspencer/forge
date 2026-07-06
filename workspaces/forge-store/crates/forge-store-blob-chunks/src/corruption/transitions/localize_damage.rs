use crate::{
    BlobChunkOrdinal, BlobCorruptedChunkLocalization, BlobCorruptionDenial,
    BlobCorruptionReferenceEdges, BlobDamageCase, BlobStreamingContentFrontier,
    BlobStreamingReadRequest, BlobVisibleGeneration,
};
use crate::corruption::classification::{
    classify_damage_case_from_detection_context,
    classify_localization_eligibility_from_frontier_match, damage_case_for_localization_denial,
    LocalizationEligibilityCase,
};
use crate::corruption::receipt_construction::construct_localization_receipt;
use crate::corruption::types::{BlobCorruptionDetectionSource, BlobCorruptionPlacementClass};
use crate::corruption::verification::verify_generation_frontier_match;
use crate::corruption::types::BlobCorruptionReferenceSharingScope;
use crate::BlobCorruptionCounterSnapshot;

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
    let damage_case = classify_damage_case_from_detection_context(
        BlobCorruptionDetectionSource::VerifiedRead,
        placement_class,
    );
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
    let damage_case = classify_damage_case_from_detection_context(source, placement_class);
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
    let ordinal_in_frontier = frontier
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .any(|leaf| leaf.ordinal() == ordinal);
    let eligibility = classify_localization_eligibility_from_frontier_match(true, ordinal_in_frontier);
    if !matches!(eligibility, LocalizationEligibilityCase::FrontierMatched) {
        let _classified = damage_case_for_localization_denial(eligibility);
        return Err(assemble_localization_denial(eligibility, ordinal));
    }
    let leaf = frontier
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .find(|leaf| leaf.ordinal() == ordinal)
        .expect("ordinal presence verified before localization");
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
    match case {
        LocalizationEligibilityCase::OrdinalNotInFrontier => {
            BlobCorruptionDenial::CorruptOrdinalNotInPublishedFrontier {
                ordinal,
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            }
        }
        LocalizationEligibilityCase::GenerationFrontierMismatch => {
            BlobCorruptionDenial::GenerationFrontierMismatch {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            }
        }
        LocalizationEligibilityCase::FrontierMatched => unreachable!("matched frontier denies nothing"),
    }
}