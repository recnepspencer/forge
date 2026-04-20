use crate::backend::records::StoreState;
use crate::failure::StoreError;

impl StoreState {
    pub fn verify_integrity(&self) -> Result<(), StoreError> {
        self.verify_wal_record_family()?;
        self.verify_commit_record_family()?;
        self.verify_branch_record_family()?;
        self.verify_support_record_family()?;
        self.verify_cursor_record_family()?;
        self.verify_live_query_record_family()?;
        self.verify_retention_record_family()?;
        self.verify_maintenance_record_family()?;
        self.verify_delta_record_family()?;
        self.verify_layout_record_family()?;
        self.verify_bulk_record_family()?;
        self.verify_snapshot_record_family()?;
        self.verify_tiering_record_family()?;
        Ok(())
    }

    pub fn verify_integrity_for_durable_recovery(&self) -> Result<(), StoreError> {
        self.verify_wal_record_family()?;
        self.verify_commit_record_family()?;
        self.verify_branch_record_family()?;
        self.verify_live_query_record_family()?;
        self.verify_retention_record_family()?;
        self.verify_maintenance_record_family()?;
        self.verify_delta_record_family()?;
        self.verify_layout_record_family()?;
        self.verify_bulk_record_family()?;
        self.verify_snapshot_record_family()?;
        self.verify_tiering_record_family()?;
        Ok(())
    }
}
