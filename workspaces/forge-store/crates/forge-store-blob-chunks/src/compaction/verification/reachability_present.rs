use crate::compaction::classification::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionIntent;

pub(crate) fn require_reachability_present(
    intent: &BlobCompactionIntent,
) -> Option<CompactionEligibilityCase> {
    if intent.reachability().is_some() {
        None
    } else {
        Some(CompactionEligibilityCase::MissingReachabilityProof)
    }
}