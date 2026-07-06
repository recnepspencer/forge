use crate::compaction::classification::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionReadHold;
use forge_store_physical_isolation::CompactionReadInterlockPlan;

pub(crate) fn require_read_hold_matches_physical(
    read_hold: BlobCompactionReadHold,
    physical: &CompactionReadInterlockPlan,
) -> Option<CompactionEligibilityCase> {
    let Some(receipt) = read_hold.released_receipt() else {
        return Some(CompactionEligibilityCase::ActiveReadHold);
    };
    let release = receipt.read_plan_release();
    if release.root() == physical.protected().root()
        && release.footprint_basis() == physical.protected().footprint_basis()
    {
        None
    } else {
        Some(CompactionEligibilityCase::ReadHoldPlanMismatch)
    }
}