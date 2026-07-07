use crate::dedupe::evidence::{
    BlobChunkCanonicalEquivalence, BlobChunkDedupeByteComparison, BlobChunkDedupeCandidate,
};
use crate::dedupe::verification::{verify_foundational_equivalence, verify_security_scope_match};
use crate::{
    BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCollisionPosture,
    BlobChunkDedupeCounterSnapshot, BlobChunkDedupePolicy, BlobChunkDedupeReceipt,
};
use forge_proof::TransitionOutcome;

pub(crate) fn construct_same_identity_equivalence_receipt(
    candidate: BlobChunkDedupeCandidate,
    policy: BlobChunkDedupePolicy,
    equivalence: BlobChunkCanonicalEquivalence,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    TransitionOutcome::success(BlobChunkDedupeReceipt::from_admitted_equivalence(
        candidate.content_digest.digest().clone(),
        candidate.security_metadata,
        policy,
        equivalence,
        BlobChunkDedupeCollisionPosture::VerifiedEquivalent,
        counters.record_same_scope_admission(),
    ))
}

pub(crate) fn construct_cross_identity_equivalence_receipt(
    existing: BlobChunkDedupeCandidate,
    candidate: BlobChunkDedupeCandidate,
    policy: BlobChunkDedupePolicy,
    equivalence: BlobChunkCanonicalEquivalence,
    byte_comparison: &BlobChunkDedupeByteComparison,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    if let Some(denial) =
        verify_foundational_equivalence(&existing, &candidate, &equivalence, counters)
    {
        return denial;
    }
    if let Some(denial) = verify_security_scope_match(&existing, &candidate, counters) {
        return denial;
    }
    TransitionOutcome::success(BlobChunkDedupeReceipt::from_admitted_equivalence(
        candidate.content_digest.digest().clone(),
        candidate.security_metadata,
        policy,
        equivalence,
        BlobChunkDedupeCollisionPosture::VerifiedEquivalent,
        byte_comparison
            .counters()
            .record_equivalence_comparison()
            .record_same_scope_admission(),
    ))
}
