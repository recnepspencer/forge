use crate::compaction::classification::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionIntent;

pub(crate) fn require_cold_readiness(
    intent: &BlobCompactionIntent,
) -> Option<CompactionEligibilityCase> {
    if intent.cold().permits_compaction() {
        None
    } else {
        Some(CompactionEligibilityCase::UnavailableColdChunk)
    }
}