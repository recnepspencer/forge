use crate::protocols::compaction_visibility::{
    map_compaction_case, map_lsm_execution_case, map_lsm_maintenance_case, map_lsm_membership_case,
};

use super::{CompactionVisibilityMappedOwnerCase, CompactionVisibilityOwnerCase};

pub fn current_compaction_visibility_owner_cases(
) -> impl Iterator<Item = CompactionVisibilityOwnerCase> {
    let membership = worth_store_lsm_authority::lsm_membership_owner_case_inventory()
        .map(|case| CompactionVisibilityOwnerCase::LsmMembership(case.id()));
    let execution = worth_store_layout_indexes::lsm_execution_owner_case_inventory()
        .map(|case| CompactionVisibilityOwnerCase::LsmExecution(case.id()));
    let maintenance = worth_store_layout_indexes::lsm_maintenance_owner_case_inventory()
        .map(|case| CompactionVisibilityOwnerCase::LsmMaintenance(case.id()));
    let compaction = worth_store_physical_isolation::compaction_owner_case_inventory()
        .map(|case| CompactionVisibilityOwnerCase::PhysicalCompaction(case.id()));

    membership
        .chain(execution)
        .chain(maintenance)
        .chain(compaction)
}

pub fn current_compaction_visibility_mappings(
) -> impl Iterator<Item = CompactionVisibilityMappedOwnerCase> {
    current_compaction_visibility_owner_cases().map(|owner_case| match owner_case {
        CompactionVisibilityOwnerCase::LsmMembership(case) => map_lsm_membership_case(case),
        CompactionVisibilityOwnerCase::LsmExecution(case) => map_lsm_execution_case(case),
        CompactionVisibilityOwnerCase::LsmMaintenance(case) => map_lsm_maintenance_case(case),
        CompactionVisibilityOwnerCase::PhysicalCompaction(case) => map_compaction_case(case),
    })
}
