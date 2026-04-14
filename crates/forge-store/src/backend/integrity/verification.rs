use crate::backend::records::StoreState;
use crate::failure::StoreError;

impl StoreState {
    pub fn verify_integrity(&self) -> Result<(), StoreError> {
        self.verify_wal_record_family()?;
        self.verify_commit_record_family()?;
        self.verify_branch_record_family()?;
        Ok(())
    }
}
