use crate::history::data::BranchId;

use super::HistorySubsystem;

const MAX_RETIRED_BRANCH_NAMES: usize = 65_536;

impl HistorySubsystem {
    pub(crate) fn remove_branch_cell(
        &self,
        branch_id: &BranchId,
    ) -> Option<crate::branch::RelationalBranchReferenceCell> {
        self.branch_cells.remove(branch_id)
    }

    pub(crate) fn reserve_branch_name_retirement(&self, branch_id: BranchId) -> Result<(), ()> {
        self.branch_cells
            .reserve_name_retirement(branch_id, MAX_RETIRED_BRANCH_NAMES)
    }

    #[cfg(test)]
    fn fill_retired_branch_name_capacity_for_test(&mut self) {
        let mut ordinal = 0_u64;
        loop {
            if self
                .branch_cells
                .reserve_name_retirement(
                    BranchId(format!("__retired-branch-capacity-proof-{ordinal}")),
                    MAX_RETIRED_BRANCH_NAMES,
                )
                .is_err()
            {
                break;
            }
            ordinal = ordinal
                .checked_add(1)
                .expect("retired branch capacity proof ordinal remains bounded");
        }
    }
}

#[cfg(test)]
impl crate::runtime::RelationalRuntime {
    pub(crate) fn fill_retired_branch_name_capacity_for_test(&mut self) {
        self.history.fill_retired_branch_name_capacity_for_test();
    }
}
