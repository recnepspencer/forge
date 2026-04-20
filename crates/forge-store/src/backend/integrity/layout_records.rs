mod chunk_membership;
mod materialization;
mod scope_membership;
mod seed;
mod structural_block;

use crate::{backend::records::StoreState, failure::StoreError};

impl StoreState {
    pub fn verify_layout_record_family(&self) -> Result<(), StoreError> {
        for (stored_key, record) in &self.milestone_6_layout_materialization_records {
            self.verify_layout_materialization_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_commit_coupled_layout_seed_records {
            self.verify_commit_coupled_layout_seed_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_scope_slice_membership_records {
            self.verify_scope_slice_membership_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_chunk_membership_records {
            self.verify_chunk_membership_record(stored_key, record)?;
        }
        for (stored_key, record) in &self.milestone_6_structural_block_records {
            self.verify_structural_block_record(stored_key, record)?;
        }
        Ok(())
    }
}
