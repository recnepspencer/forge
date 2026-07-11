use crate::compaction::classification::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionIntent;

pub(crate) fn require_pacing_admission(
    intent: &BlobCompactionIntent,
) -> Option<CompactionEligibilityCase> {
    if intent.pacing().supports_compaction() && intent.pacing().io_readmission_satisfied() {
        None
    } else {
        Some(CompactionEligibilityCase::UnsupportedSchedulerPacing)
    }
}
