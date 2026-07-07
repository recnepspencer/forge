use crate::compaction::classification::CompactionEligibilityCase;
use crate::{BlobChunkReachabilityProofSet, LifecycleReceipt};

pub(crate) fn require_lifecycle_reachability(
    lifecycle: &LifecycleReceipt,
    reachability: &BlobChunkReachabilityProofSet,
) -> Option<CompactionEligibilityCase> {
    if !reachability.matches_lifecycle_declaration(lifecycle.declaration()) {
        return Some(CompactionEligibilityCase::LifecycleReachabilityMismatch);
    }
    if reachability.protected_holds().is_empty() {
        None
    } else {
        Some(CompactionEligibilityCase::ActiveReadHold)
    }
}
