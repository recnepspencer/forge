use crate::compaction::classification::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionIntent;

pub(crate) fn require_physical_interlock(
    intent: &BlobCompactionIntent,
) -> Option<CompactionEligibilityCase> {
    if intent.physical().admitted().is_some() {
        None
    } else {
        Some(CompactionEligibilityCase::PhysicalInterlockDenied)
    }
}