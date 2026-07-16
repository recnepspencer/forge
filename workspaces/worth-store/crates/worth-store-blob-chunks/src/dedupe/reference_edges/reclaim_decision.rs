use crate::BlobChunkDedupeCounterSnapshot;

use super::reference_set::BlobChunkDedupeReferenceSet;
use super::released_edges::BlobChunkDedupeReferenceRelease;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobChunkDedupeReclaimDecision {
    ReclaimPermitted(Box<BlobChunkDedupeReferenceRelease>),
    ReclaimDenied(BlobChunkDedupeCounterSnapshot),
}

pub(super) fn classify_reclaim(
    set: &BlobChunkDedupeReferenceSet,
) -> BlobChunkDedupeReclaimDecision {
    if set.has_live_edges() {
        BlobChunkDedupeReclaimDecision::ReclaimDenied(
            set.counters().record_reclaim_blocked_by_reference_edge(),
        )
    } else {
        BlobChunkDedupeReclaimDecision::ReclaimPermitted(Box::new(
            BlobChunkDedupeReferenceRelease::snapshot(set),
        ))
    }
}
