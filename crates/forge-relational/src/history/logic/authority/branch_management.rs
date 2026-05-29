use crate::history::data::{BranchCreateError, BranchId};

use super::HistoryAuthority;

impl<'runtime> HistoryAuthority<'runtime> {
    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        if self.runtime.history.branch_heads.contains_key(&new_branch) {
            return Err(BranchCreateError::branch_already_exists());
        }
        let Some(source_head) = self.runtime.history.branch_heads.get(from_branch).cloned() else {
            return Err(BranchCreateError::source_branch_missing());
        };
        self.runtime
            .history
            .branch_heads
            .insert(new_branch, source_head.clone());
        self.runtime
            .visibility_pins()
            .move_branch_head_visibility_residency(
                None,
                source_head.as_ref().map(|head| head.version_id),
            );
        if let Some(source_head) = source_head {
            self.runtime
                .visibility_pins()
                .pin_branch_version(source_head.version_id);
        }
        Ok(())
    }
}
