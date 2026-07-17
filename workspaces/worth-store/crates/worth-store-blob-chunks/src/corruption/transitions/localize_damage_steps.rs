use crate::corruption::classification::{
    classify_localization_eligibility_from_frontier_match, LocalizationEligibilityCase,
};
use crate::corruption::types::BlobCorruptionReferenceSharingScope;
use crate::{
    BlobChunkOrdinal, BlobCorruptionDenial, BlobCorruptionReferenceEdges, BlobGeneration,
    BlobObjectId, BlobStreamingContentFrontier,
};

pub(crate) fn verify_ordinal_localization_eligibility(
    frontier: &BlobStreamingContentFrontier,
    ordinal: BlobChunkOrdinal,
) -> Result<(), LocalizationEligibilityCase> {
    let ordinal_in_frontier = frontier
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .any(|leaf| leaf.ordinal() == ordinal);
    let eligibility =
        classify_localization_eligibility_from_frontier_match(true, ordinal_in_frontier);
    match eligibility {
        LocalizationEligibilityCase::FrontierMatched => Ok(()),
        denial_case => Err(denial_case),
    }
}

pub(crate) fn resolve_corrupt_chunk_leaf(
    frontier: &BlobStreamingContentFrontier,
    ordinal: BlobChunkOrdinal,
) -> &crate::BlobChunkProofLeaf {
    frontier
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .find(|leaf| leaf.ordinal() == ordinal)
        .expect("ordinal presence verified before localization")
}

pub(crate) fn validate_corrupt_chunk_reference_edges(
    reference_edges: &BlobCorruptionReferenceEdges,
    object_id: &BlobObjectId,
    generation: BlobGeneration,
    frontier: &BlobStreamingContentFrontier,
    leaf: &crate::BlobChunkProofLeaf,
) -> Result<u64, BlobCorruptionDenial> {
    reference_edges.validated_edge_count_for_corrupt_chunk(
        object_id,
        generation,
        frontier.chunk_tree_root(),
        frontier.logical_content_digest(),
        leaf.identity(),
    )
}

pub(crate) const fn classify_reference_sharing_scope(
    edge_count: u64,
) -> BlobCorruptionReferenceSharingScope {
    if edge_count == 1 {
        BlobCorruptionReferenceSharingScope::SingleReference
    } else {
        BlobCorruptionReferenceSharingScope::SharedReferenceEdges
    }
}
