use crate::history::data::BranchId;

use super::HistorySubsystem;

const MAX_RETIRED_BRANCH_NAMES: usize = 65_536;

impl HistorySubsystem {
    pub(crate) fn remove_branch_cell(
        &mut self,
        branch_id: &BranchId,
    ) -> Option<crate::branch::RelationalBranchReferenceCell> {
        self.branch_cells.remove(branch_id)
    }

    pub(crate) fn branch_name_is_retired(&self, branch_id: &BranchId) -> bool {
        self.retired_branch_names.contains(branch_id)
    }

    fn can_retire_branch_name(&self, branch_id: &BranchId) -> bool {
        self.retired_branch_names.contains(branch_id)
            || self.retired_branch_names.len() < MAX_RETIRED_BRANCH_NAMES
    }

    pub(crate) fn reserve_branch_name_retirement(&mut self, branch_id: BranchId) -> Result<(), ()> {
        if !self.can_retire_branch_name(&branch_id) {
            return Err(());
        }
        self.retired_branch_names.insert(branch_id);
        Ok(())
    }

    #[cfg(test)]
    fn fill_retired_branch_name_capacity_for_test(&mut self) {
        let mut ordinal = 0_u64;
        while self.retired_branch_names.len() < MAX_RETIRED_BRANCH_NAMES {
            self.retired_branch_names.insert(BranchId(format!(
                "__retired-branch-capacity-proof-{ordinal}"
            )));
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
