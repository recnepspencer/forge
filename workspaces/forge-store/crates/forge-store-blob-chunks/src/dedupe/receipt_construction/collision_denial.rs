use crate::dedupe::evidence::BlobChunkDedupeCandidate;
use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::{
    BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeByteComparison, BlobChunkDedupeCollisionPosture,
};
use forge_proof::TransitionOutcome;

pub(crate) fn construct_digest_collision_denial(
    existing: BlobChunkDedupeCandidate,
    candidate: BlobChunkDedupeCandidate,
    byte_comparison: BlobChunkDedupeByteComparison,
) -> BlobChunkDedupeAdmissionOutcome {
    let bytes_compared = byte_comparison.bytes_compared();
    let collision_counters = byte_comparison.counters_for_collision_denial();
    let receipt = BlobChunkCollisionVerificationReceipt::from_verified_identity_mismatch(
        existing.proof,
        candidate.proof,
        candidate.content_digest,
        candidate.security_metadata,
        bytes_compared,
        collision_counters,
    );
    TransitionOutcome::denied(crate::BlobChunkDedupeAdmissionDenial::DigestCollisionDenied {
        posture: BlobChunkDedupeCollisionPosture::DigestCollisionDenied,
        counters: receipt.counters(),
        receipt,
    })
}