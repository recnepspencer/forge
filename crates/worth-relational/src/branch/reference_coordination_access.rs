use std::sync::Arc;

use super::RelationalBranchReferenceCell;

impl RelationalBranchReferenceCell {
    pub(crate) fn coordination(
        &self,
    ) -> &Arc<super::super::coordination::RelationalBranchCoordinationCell> {
        &self.coordination
    }
}
