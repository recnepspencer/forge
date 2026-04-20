use crate::backend::records::{
    MaintenanceBatchRecord, MaintenanceCheckpointRecord, MaintenanceDeclarationRecord,
    MaintenanceExecutionRecord, StoreState,
};
use crate::failure::{StoreError, StoreErrorKind};

impl StoreState {
    fn verify_maintenance_declaration_record(
        &self,
        record: &MaintenanceDeclarationRecord,
    ) -> Result<(), StoreError> {
        if record.artifact_id != record.declaration.id().as_str() {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceAdmissionViolation,
                format!(
                    "maintenance declaration record `{}` drifted from declaration identity `{}`",
                    record.artifact_id,
                    record.declaration.id().as_str()
                ),
            ));
        }
        if record.declaration_class != record.declaration.class() {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceAdmissionViolation,
                format!(
                    "maintenance declaration `{}` drifted from its declared class",
                    record.artifact_id
                ),
            ));
        }
        if !self
            .maintenance_batch_records
            .contains_key(&record.batch_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceDeclarationMissing,
                format!(
                    "maintenance declaration `{}` referenced missing batch `{}`",
                    record.artifact_id, record.batch_id
                ),
            ));
        }
        let batch = self
            .maintenance_batch_records
            .get(&record.batch_id)
            .expect("batch existence checked above");
        if !batch
            .declaration_ids
            .iter()
            .any(|declaration_id| declaration_id == &record.artifact_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceAdmissionViolation,
                format!(
                    "maintenance declaration `{}` was persisted under batch `{}` but batch membership does not include it",
                    record.artifact_id, record.batch_id
                ),
            ));
        }
        Ok(())
    }

    fn verify_maintenance_execution_record(
        &self,
        record: &MaintenanceExecutionRecord,
    ) -> Result<(), StoreError> {
        if record.artifact_id != record.declaration_id {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceLifecycleViolation,
                format!(
                    "maintenance execution record `{}` drifted from declaration id `{}`",
                    record.artifact_id, record.declaration_id
                ),
            ));
        }
        if !self
            .maintenance_declaration_records
            .contains_key(&record.declaration_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceDeclarationMissing,
                format!(
                    "maintenance execution record `{}` referenced missing declaration `{}`",
                    record.artifact_id, record.declaration_id
                ),
            ));
        }
        Ok(())
    }

    fn verify_maintenance_batch_record(
        &self,
        record: &MaintenanceBatchRecord,
    ) -> Result<(), StoreError> {
        if record.declaration_count != record.declaration_ids.len() as u64 {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceAdmissionViolation,
                format!(
                    "maintenance batch `{}` declaration count drifted from stored declaration ids",
                    record.artifact_id
                ),
            ));
        }
        for declaration_id in &record.declaration_ids {
            let declaration = match self.maintenance_declaration_records.get(declaration_id) {
                Some(declaration) => declaration,
                None => {
                    return Err(StoreError::new(
                        StoreErrorKind::MaintenanceDeclarationMissing,
                        format!(
                            "maintenance batch `{}` referenced missing declaration `{declaration_id}`",
                            record.artifact_id
                        ),
                    ));
                }
            };
            if declaration.batch_id != record.artifact_id {
                return Err(StoreError::new(
                    StoreErrorKind::MaintenanceAdmissionViolation,
                    format!(
                        "maintenance batch `{}` referenced declaration `{declaration_id}` owned by different batch `{}`",
                        record.artifact_id, declaration.batch_id
                    ),
                ));
            }
        }
        let actual_batch_members = self
            .maintenance_declaration_records
            .values()
            .filter(|declaration| declaration.batch_id == record.artifact_id)
            .count() as u64;
        if actual_batch_members != record.declaration_count {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceAdmissionViolation,
                format!(
                    "maintenance batch `{}` declared {} members but {} declarations point back to it",
                    record.artifact_id, record.declaration_count, actual_batch_members
                ),
            ));
        }
        Ok(())
    }

    fn verify_maintenance_checkpoint_record(
        &self,
        record: &MaintenanceCheckpointRecord,
    ) -> Result<(), StoreError> {
        if !self
            .maintenance_execution_records
            .contains_key(&record.declaration_id)
        {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceCheckpointViolation,
                format!(
                    "maintenance checkpoint `{}` referenced missing declaration `{}`",
                    record.artifact_id, record.declaration_id
                ),
            ));
        }
        if record.completed_phase.trim().is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceCheckpointViolation,
                format!(
                    "maintenance checkpoint `{}` must publish a non-empty completed phase",
                    record.artifact_id
                ),
            ));
        }
        Ok(())
    }

    pub fn verify_maintenance_record_family(&self) -> Result<(), StoreError> {
        for record in self.maintenance_batch_records.values() {
            self.verify_maintenance_batch_record(record)?;
        }
        for record in self.maintenance_declaration_records.values() {
            self.verify_maintenance_declaration_record(record)?;
        }
        for record in self.maintenance_execution_records.values() {
            self.verify_maintenance_execution_record(record)?;
        }
        for record in self.maintenance_checkpoint_records.values() {
            self.verify_maintenance_checkpoint_record(record)?;
        }
        Ok(())
    }
}
