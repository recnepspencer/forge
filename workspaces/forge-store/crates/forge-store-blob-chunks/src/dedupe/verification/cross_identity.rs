use crate::dedupe::evidence::{BlobChunkDedupeByteComparison, BlobChunkDedupeCandidate};
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot,
    BlobChunkRootCanonicalComparison,
};
use forge_proof::TransitionOutcome;

pub(crate) fn verify_cross_identity_comparisons(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    root_comparison: &BlobChunkRootCanonicalComparison,
    byte_comparison: &BlobChunkDedupeByteComparison,
    counters: BlobChunkDedupeCounterSnapshot,
) -> Option<BlobChunkDedupeAdmissionOutcome> {
    if !byte_comparison.matches_candidate_identities(&existing.identity, &candidate.identity) {
        return Some(TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::UnboundByteComparison {
                counters: counters.record_cross_scope_denial(),
            },
        ));
    }
    if !root_comparison.matches_candidate_identities(&existing.identity, &candidate.identity) {
        return Some(TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::UnboundRootCanonicalComparison {
                counters: counters.record_cross_scope_denial(),
            },
        ));
    }
    None
}