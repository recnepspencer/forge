use super::{RelationalBranchCellDenial, RelationalBranchReferenceCell};

impl RelationalBranchReferenceCell {
    pub(crate) fn retain_head(&mut self) -> Result<(), RelationalBranchCellDenial> {
        self.head_retention_obligations = self
            .head_retention_obligations
            .checked_add(1)
            .ok_or(RelationalBranchCellDenial::RetentionOverflow)?;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn set_head_retention_obligations_for_test(&mut self, obligations: u32) {
        self.head_retention_obligations = obligations;
    }
}
