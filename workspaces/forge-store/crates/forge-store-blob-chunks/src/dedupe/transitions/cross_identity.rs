use crate::dedupe::evidence::{
    BlobChunkCanonicalEquivalence, BlobChunkDedupeByteComparison, BlobChunkDedupeCandidate,
};
use crate::dedupe::receipt_construction::{collision_denial, equivalence_receipt};
use crate::dedupe::verification::verify_cross_identity_comparisons;
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot,
    BlobChunkDedupePolicy, BlobChunkRootCanonicalComparison,
};
use forge_proof::TransitionOutcome;

pub(crate) fn admit_cross_identity_case(
    existing: BlobChunkDedupeCandidate,
    candidate: BlobChunkDedupeCandidate,
    root_comparison: Option<BlobChunkRootCanonicalComparison>,
    byte_comparison: Option<BlobChunkDedupeByteComparison>,
    policy: BlobChunkDedupePolicy,
    equivalence: BlobChunkCanonicalEquivalence,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    let Some(root_comparison) = root_comparison else {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CanonicalRootComparisonRequired {
                counters: counters.record_collision_probe(),
            },
        );
    };
    let Some(byte_comparison) = byte_comparison else {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::ChunkByteComparisonRequired {
                counters: counters.record_collision_probe(),
            },
        );
    };
    if let Some(denial) = verify_cross_identity_comparisons(
        &existing,
        &candidate,
        &root_comparison,
        &byte_comparison,
        counters,
    ) {
        return denial;
    }
    if root_comparison.is_equivalent() && byte_comparison.is_equivalent() {
        return equivalence_receipt::construct_cross_identity_equivalence_receipt(
            existing,
            candidate,
            policy,
            equivalence,
            &byte_comparison,
            counters,
        );
    }
    collision_denial::construct_digest_collision_denial(existing, candidate, byte_comparison)
}