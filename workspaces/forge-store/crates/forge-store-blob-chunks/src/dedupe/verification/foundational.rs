use crate::dedupe::evidence::{BlobChunkCanonicalEquivalence, BlobChunkDedupeCandidate};
use crate::{BlobChunkDedupeAdmissionDenial, BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot};
use forge_proof::TransitionOutcome;

pub(crate) fn verify_foundational_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    equivalence: &BlobChunkCanonicalEquivalence,
    counters: BlobChunkDedupeCounterSnapshot,
) -> Option<BlobChunkDedupeAdmissionOutcome> {
    if equivalence.matches_candidates(existing, candidate) {
        None
    } else {
        Some(TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::UnboundFoundationalEquivalence {
                counters: counters.record_cross_scope_denial(),
            },
        ))
    }
}