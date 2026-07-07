use crate::compaction::classification::CompactionEligibilityCase;
use crate::{AdmittedBlobPlacement, LifecycleReceipt};

pub(crate) fn require_lifecycle_placement(
    lifecycle: &LifecycleReceipt,
    placement: &AdmittedBlobPlacement,
) -> Option<CompactionEligibilityCase> {
    if placement.matches_reachability(lifecycle.reachability()) {
        None
    } else {
        Some(CompactionEligibilityCase::LifecyclePlacementMismatch)
    }
}
