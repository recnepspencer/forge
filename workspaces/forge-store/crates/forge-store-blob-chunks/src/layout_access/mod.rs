mod behavior;
mod blob_layout_closeout;
mod blob_object_family;
mod chunk_tree_family;
mod compaction_family;
mod dedupe_family;
mod denial;
mod evidence;
mod quarantine_family;
mod reachability_family;
mod reclaim_family;
mod retention_family;
mod streaming_family;

pub use behavior::{BlobLayoutCorruptionBehavior, BlobLayoutScopeSafeAbsenceBehavior};
pub use blob_layout_closeout::BlobLayoutCloseout;
pub use blob_object_family::{
    reject_chunk_tree_root_as_blob_object_layout_authority, BlobGenerationPublicationLayoutReport,
    BlobObjectLayoutReport,
};
pub use chunk_tree_family::{
    reject_streaming_frontier_as_chunk_tree_layout_authority, ChunkTreeLayoutReport,
    StoredChunkLookupLayoutReport,
};
pub use compaction_family::CompactionLayoutReport;
pub use dedupe_family::DedupeLayoutReport;
pub use denial::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind};
pub use evidence::BlobLayoutAccessPathEvidence;
pub use quarantine_family::QuarantineLayoutReport;
pub use reachability_family::ReachabilityLayoutReport;
pub use reclaim_family::ReclaimLayoutReport;
pub use retention_family::RetentionLayoutReport;
pub use streaming_family::{
    reject_full_blob_buffer_as_streaming_layout_authority, StreamingLayoutReport,
    StreamingResumeLayoutReport,
};

use forge_store_budgets::CounterEvidenceStrength;

use crate::BlobStreamingReadCounterSnapshot;

pub(super) const fn read_counters_are_exact(counters: BlobStreamingReadCounterSnapshot) -> bool {
    matches!(counters.counter_strength(), CounterEvidenceStrength::Exact)
}

#[cfg(test)]
mod tests;
