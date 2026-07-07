use crate::compaction::classification::CompactionEligibilityCase;
use crate::{BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference};

pub(crate) fn require_dedupe_edges(
    references: &[BlobChunkRegisteredDedupeReference],
    reachability: &BlobChunkReachabilityProofSet,
) -> Option<CompactionEligibilityCase> {
    for reference in references {
        if reference.security_metadata() != reachability.security_metadata() {
            return Some(CompactionEligibilityCase::DedupeScopeMismatch);
        }
        if !reachability
            .reachable_chunks()
            .iter()
            .any(|chunk| reference.contains_chunk_identity(chunk))
        {
            return Some(CompactionEligibilityCase::StaleDedupeReference);
        }
    }
    None
}
