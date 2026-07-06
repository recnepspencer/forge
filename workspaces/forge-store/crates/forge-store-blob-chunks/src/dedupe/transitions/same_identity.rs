use crate::dedupe::evidence::{BlobChunkCanonicalEquivalence, BlobChunkDedupeCandidate};
use crate::dedupe::receipt_construction::equivalence_receipt;
use crate::dedupe::verification::{verify_foundational_equivalence, verify_security_scope_match};
use crate::{
    BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot, BlobChunkDedupePolicy,
};

pub(crate) fn admit_same_identity_case(
    existing: BlobChunkDedupeCandidate,
    candidate: BlobChunkDedupeCandidate,
    policy: BlobChunkDedupePolicy,
    equivalence: BlobChunkCanonicalEquivalence,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    if let Some(denial) = verify_foundational_equivalence(&existing, &candidate, &equivalence, counters) {
        return denial;
    }
    if let Some(denial) = verify_security_scope_match(&existing, &candidate, counters) {
        return denial;
    }
    equivalence_receipt::construct_same_identity_equivalence_receipt(
        candidate, policy, equivalence, counters,
    )
}