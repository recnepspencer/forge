use crate::compaction::classification::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionIntent;

pub(crate) fn require_quarantine_absent(
    intent: &BlobCompactionIntent,
) -> Option<CompactionEligibilityCase> {
    if intent.quarantine_holds().is_empty() {
        None
    } else {
        Some(CompactionEligibilityCase::QuarantineHold)
    }
}