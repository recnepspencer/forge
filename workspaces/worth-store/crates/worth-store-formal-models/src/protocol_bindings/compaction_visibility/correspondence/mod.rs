mod execution;
mod maintenance;
mod membership;
mod physical;

use crate::protocols::compaction_visibility::CompactionVisibilityAction;

use super::CompactionVisibilityOwnerCase;

pub(super) fn expected_action_for_owner_case(
    owner_case: CompactionVisibilityOwnerCase,
) -> CompactionVisibilityAction {
    match owner_case {
        CompactionVisibilityOwnerCase::LsmMembership(case) => membership::expected_action(case),
        CompactionVisibilityOwnerCase::LsmExecution(case) => execution::expected_action(case),
        CompactionVisibilityOwnerCase::LsmMaintenance(case) => maintenance::expected_action(case),
        CompactionVisibilityOwnerCase::PhysicalCompaction(case) => physical::expected_action(case),
    }
}
