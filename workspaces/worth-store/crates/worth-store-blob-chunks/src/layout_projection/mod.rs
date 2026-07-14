mod behavior;
mod blob_object;
mod chunk_tree;
mod compaction;
mod counters;
mod dedupe;
mod denial;
mod quarantine;
mod reachability;
mod reclaim;
mod retention;
mod streaming;

pub use behavior::{BlobLayoutCorruptionBehavior, BlobLayoutScopeSafeAbsenceBehavior};
pub use blob_object::{
    reject_chunk_tree_root_as_blob_object_layout_authority, BlobGenerationPublicationLayoutReport,
    BlobObjectLayoutReport,
};
pub use chunk_tree::{
    reject_streaming_frontier_as_chunk_tree_layout_authority, ChunkTreeLayoutReport,
    StoredChunkLookupLayoutReport,
};
pub use compaction::CompactionLayoutReport;
pub use counters::{BlobLayoutAccessPathEvidence, BlobLayoutAccessShape};
pub use dedupe::DedupeLayoutReport;
pub use denial::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind};
pub use reachability::ReachabilityLayoutReport;
pub use reclaim::ReclaimLayoutReport;
pub use retention::RetentionLayoutReport;
pub use streaming::{
    reject_full_blob_buffer_as_streaming_layout_authority, StreamingLayoutReport,
    StreamingResumeLayoutReport,
};

use worth_store_budgets::CounterEvidenceStrength;

use crate::BlobStreamingReadCounterSnapshot;

pub(super) const fn read_counters_are_exact(counters: BlobStreamingReadCounterSnapshot) -> bool {
    matches!(counters.counter_strength(), CounterEvidenceStrength::Exact)
}

#[cfg(test)]
mod tests;
