use crate::dedupe::evidence::BlobChunkDedupeCandidate;
use crate::dedupe::receipt_construction::denial_assembly;
use crate::{BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot};

pub(crate) fn verify_security_scope_match(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> Option<BlobChunkDedupeAdmissionOutcome> {
    denial_assembly::deny_scope_mismatch(existing, candidate, counters)
}