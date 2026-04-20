mod artifacts;
mod layer_record;
mod replacement;
mod shared_base;

use crate::{
    backend::records::StoreState,
    failure::StoreError,
};

impl StoreState {
    pub fn verify_delta_record_family(&self) -> Result<(), StoreError> {
        for record in self.branch_shared_base_records.values() {
            self.verify_branch_shared_base_record(record)?;
        }
        for record in self.branch_delta_layer_records.values() {
            self.verify_branch_delta_layer_record(record)?;
        }
        Ok(())
    }
}
