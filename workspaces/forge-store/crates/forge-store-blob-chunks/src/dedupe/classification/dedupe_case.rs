use crate::dedupe::evidence::BlobChunkDedupeCandidate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedupeCase {
    SameIdentity,
    CrossIdentity,
}

pub(crate) fn classify_dedupe_case(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
) -> DedupeCase {
    if existing.identity == candidate.identity {
        DedupeCase::SameIdentity
    } else {
        DedupeCase::CrossIdentity
    }
}
