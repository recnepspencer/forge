use crate::{
    BlobCorruptionCounterSnapshot, BlobCorruptionDenial, BlobStreamingContentFrontier,
    BlobVisibleGeneration,
};

use crate::corruption::classification::{classify_blob_damage_before_decode, BlobDamageEvidence};

pub(crate) fn verify_generation_frontier_match(
    visible_generation: &BlobVisibleGeneration,
    frontier: &BlobStreamingContentFrontier,
) -> Result<(), BlobCorruptionDenial> {
    let generation_matches = visible_generation.chunk_tree_root() == frontier.chunk_tree_root()
        && visible_generation.logical_content_digest() == frontier.logical_content_digest();
    if generation_matches {
        return Ok(());
    }
    let damage_case =
        classify_blob_damage_before_decode(BlobDamageEvidence::GenerationFrontierMismatch);
    Err(BlobCorruptionDenial::GenerationFrontierMismatch {
        damage_case,
        counters: BlobCorruptionCounterSnapshot::start().record_denial(),
    })
}
