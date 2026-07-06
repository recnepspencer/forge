use super::candidate::BlobChunkDedupeCandidate;
use crate::{BlobChunkDedupeAdmissionDenial, BlobChunkDedupeAdmissionOutcome, BlobChunkDedupeCounterSnapshot};
use forge_proof::TransitionOutcome;

pub(crate) fn digest_gate(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> Result<BlobChunkDedupeCounterSnapshot, BlobChunkDedupeAdmissionOutcome> {
    if existing.content_digest != candidate.content_digest {
        Err(TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::ContentDigestMismatch {
                counters: counters.record_dedupe_miss(),
            },
        ))
    } else {
        Ok(counters)
    }
}