mod checkpoints;
mod frozen_inputs;
mod plan_execution;
mod program_identity;
mod witness_index;

use crate::{backend::records::StoreState, failure::StoreError};

impl StoreState {
    pub fn verify_bulk_record_family(&self) -> Result<(), StoreError> {
        self.verify_bulk_program_identity_records()?;
        self.verify_bulk_frozen_input_records()?;
        self.verify_bulk_plan_and_witness_records()?;
        self.verify_bulk_checkpoint_records()?;
        self.verify_bulk_witness_index_records()?;
        Ok(())
    }
}
