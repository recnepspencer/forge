use crate::{
    BlobChunkCanonicalComparisonBasis, BlobChunkCanonicalEquivalence, BlobChunkDedupeCandidate,
    BlobChunkSecurityScope,
};

use super::integrity::integrity_proof_for_scope;

pub(crate) fn candidate_for_scope(scope: BlobChunkSecurityScope) -> BlobChunkDedupeCandidate {
    candidate_for_scope_with_digest(scope, "sha256:blob-s51-same-content")
}

pub(crate) fn candidate_for_bytes_and_scope(
    bytes: &[u8],
    scope: BlobChunkSecurityScope,
) -> BlobChunkDedupeCandidate {
    BlobChunkDedupeCandidate::from_integrity_proof(integrity_proof_for_scope(scope, bytes))
}

pub(crate) fn candidate_for_scope_with_digest(
    scope: BlobChunkSecurityScope,
    digest_raw: &str,
) -> BlobChunkDedupeCandidate {
    BlobChunkDedupeCandidate::from_integrity_proof(integrity_proof_for_scope(
        scope,
        digest_raw.as_bytes(),
    ))
}

pub(crate) fn canonical_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
) -> BlobChunkCanonicalEquivalence {
    BlobChunkCanonicalComparisonBasis::from_candidates(existing, candidate)
        .expect("candidate comparison basis should prepare")
        .evaluate_foundational_equivalence()
        .expect("candidate-derived equivalence should admit")
}

pub(crate) fn forced_collision_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
) -> BlobChunkCanonicalEquivalence {
    BlobChunkCanonicalEquivalence::forced_digest_collision_fixture(existing, candidate)
}
