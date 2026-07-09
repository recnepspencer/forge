use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeAdmissionOutcome,
    BlobChunkDedupeCounterSnapshot, BlobChunkDedupePolicy,
};
use worth_proof::TransitionOutcome;

pub(crate) fn verify_policy_allows_sharing(
    policy: BlobChunkDedupePolicy,
    counters: BlobChunkDedupeCounterSnapshot,
) -> Result<BlobChunkDedupeCounterSnapshot, BlobChunkDedupeAdmissionOutcome> {
    if policy.allows_same_scope_sharing() {
        Ok(counters)
    } else {
        Err(TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::DedupePolicyDenied {
                policy,
                counters: counters.record_dedupe_miss(),
            },
        ))
    }
}
