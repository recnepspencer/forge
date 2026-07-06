use crate::{BlobCorruptionDenial, BlobCorruptionCounterSnapshot, BlobStreamingContentFrontier, BlobVisibleGeneration};

use super::super::classification::{
    classify_localization_eligibility_from_frontier_match, damage_case_for_localization_denial,
    LocalizationEligibilityCase,
};

pub(crate) fn verify_generation_frontier_match(
    visible_generation: &BlobVisibleGeneration,
    frontier: &BlobStreamingContentFrontier,
) -> Result<(), BlobCorruptionDenial> {
    let generation_matches = visible_generation.chunk_tree_root() == frontier.chunk_tree_root()
        && visible_generation.logical_content_digest() == frontier.logical_content_digest();
    let case = classify_localization_eligibility_from_frontier_match(generation_matches, true);
    match case {
        LocalizationEligibilityCase::FrontierMatched => Ok(()),
        LocalizationEligibilityCase::GenerationFrontierMismatch => {
            let _damage_case = damage_case_for_localization_denial(case);
            Err(BlobCorruptionDenial::GenerationFrontierMismatch {
                counters: BlobCorruptionCounterSnapshot::start().record_denial(),
            })
        }
        LocalizationEligibilityCase::OrdinalNotInFrontier => unreachable!(
            "frontier match verification does not evaluate ordinal presence"
        ),
    }
}