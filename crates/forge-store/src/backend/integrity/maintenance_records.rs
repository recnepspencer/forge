use crate::backend::records::{
    MaintenanceBatchRecord, MaintenanceCheckpointRecord, MaintenanceDebtSummaryRecord,
    MaintenanceDeclarationRecord, MaintenanceExecutionRecord, MaintenanceLocalitySummaryRecord,
    MaintenanceQueueSummaryRecord, MaintenanceReservationSummaryRecord,
    MaintenanceResourceBudgetSummaryRecord, StoreState,
};
use crate::failure::{StoreError, StoreErrorKind};
use crate::maintenance::MaintenanceExecutionStatus;

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
        let declaration = self
            .maintenance_declaration_records
            .get(&record.declaration_id)
            .expect("declaration existence checked above");
        if let Some(lane_key) = &record.lane_key {
            if lane_key != &declaration.work_descriptor.lane_key() {
                return Err(StoreError::backend_integrity(format!(
                    "maintenance execution record `{}` drifted from descriptor lane key",
                    record.artifact_id
                )));
            }
        }
        if matches!(
            record.execution_status,
            MaintenanceExecutionStatus::Reserved | MaintenanceExecutionStatus::Started
        ) && record.resource_budget_grant.is_none()
        {
            return Err(StoreError::new(
                StoreErrorKind::MaintenanceLifecycleViolation,
                format!(
                    "maintenance execution record `{}` entered {:?} without a resource budget grant",
                    record.artifact_id, record.execution_status
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

    fn verify_queue_summary_record(
        &self,
        record: &MaintenanceQueueSummaryRecord,
        expected: &MaintenanceQueueSummaryRecord,
    ) -> Result<(), StoreError> {
        if record != expected {
            return Err(StoreError::backend_integrity(format!(
                "maintenance queue summary `{}` drifted from declaration/execution truth",
                record.artifact_id
            )));
        }
        Ok(())
    }

    fn verify_locality_summary_record(
        &self,
        record: &MaintenanceLocalitySummaryRecord,
        expected: &MaintenanceLocalitySummaryRecord,
    ) -> Result<(), StoreError> {
        if record != expected {
            return Err(StoreError::backend_integrity(format!(
                "maintenance locality summary `{}` drifted from declaration/execution truth",
                record.artifact_id
            )));
        }
        Ok(())
    }

    fn verify_reservation_summary_record(
        &self,
        record: &MaintenanceReservationSummaryRecord,
        expected: &MaintenanceReservationSummaryRecord,
    ) -> Result<(), StoreError> {
        if record != expected {
            return Err(StoreError::backend_integrity(format!(
                "maintenance reservation summary `{}` drifted from declaration/execution truth",
                record.artifact_id
            )));
        }
        Ok(())
    }

    fn verify_resource_budget_summary_record(
        &self,
        record: &MaintenanceResourceBudgetSummaryRecord,
        expected: &MaintenanceResourceBudgetSummaryRecord,
    ) -> Result<(), StoreError> {
        if record != expected {
            return Err(StoreError::backend_integrity(format!(
                "maintenance resource budget summary `{}` drifted from declaration/execution truth",
                record.artifact_id
            )));
        }
        Ok(())
    }

    fn verify_debt_summary_record(
        &self,
        record: &MaintenanceDebtSummaryRecord,
        expected: &MaintenanceDebtSummaryRecord,
    ) -> Result<(), StoreError> {
        if record != expected {
            return Err(StoreError::backend_integrity(format!(
                "maintenance debt summary `{}` drifted from declaration/execution truth",
                record.artifact_id
            )));
        }
        Ok(())
    }

    fn verify_maintenance_summary_records(&self) -> Result<(), StoreError> {
        let mut rebuilt = self.clone();
        crate::backend::maintenance::summaries::refresh_scheduler_summaries(&mut rebuilt);

        if self.maintenance_queue_summary_records.len()
            != rebuilt.maintenance_queue_summary_records.len()
            || self.maintenance_locality_summary_records.len()
                != rebuilt.maintenance_locality_summary_records.len()
            || self.maintenance_reservation_summary_records.len()
                != rebuilt.maintenance_reservation_summary_records.len()
            || self.maintenance_resource_budget_summary_records.len()
                != rebuilt.maintenance_resource_budget_summary_records.len()
            || self.maintenance_debt_summary_records.len()
                != rebuilt.maintenance_debt_summary_records.len()
        {
            return Err(StoreError::backend_integrity(
                "maintenance summary record families drifted from rebuilt scheduler truth",
            ));
        }

        for (artifact_id, record) in &self.maintenance_queue_summary_records {
            let expected = rebuilt
                .maintenance_queue_summary_records
                .get(artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "maintenance queue summary `{artifact_id}` is missing from rebuilt truth"
                    ))
                })?;
            self.verify_queue_summary_record(record, expected)?;
        }
        for (artifact_id, record) in &self.maintenance_locality_summary_records {
            let expected = rebuilt
                .maintenance_locality_summary_records
                .get(artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "maintenance locality summary `{artifact_id}` is missing from rebuilt truth"
                    ))
                })?;
            self.verify_locality_summary_record(record, expected)?;
        }
        for (artifact_id, record) in &self.maintenance_reservation_summary_records {
            let expected = rebuilt
                .maintenance_reservation_summary_records
                .get(artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "maintenance reservation summary `{artifact_id}` is missing from rebuilt truth"
                    ))
                })?;
            self.verify_reservation_summary_record(record, expected)?;
        }
        for (artifact_id, record) in &self.maintenance_resource_budget_summary_records {
            let expected = rebuilt
                .maintenance_resource_budget_summary_records
                .get(artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "maintenance resource budget summary `{artifact_id}` is missing from rebuilt truth"
                    ))
                })?;
            self.verify_resource_budget_summary_record(record, expected)?;
        }
        for (artifact_id, record) in &self.maintenance_debt_summary_records {
            let expected = rebuilt
                .maintenance_debt_summary_records
                .get(artifact_id)
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "maintenance debt summary `{artifact_id}` is missing from rebuilt truth"
                    ))
                })?;
            self.verify_debt_summary_record(record, expected)?;
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
        self.verify_maintenance_summary_records()?;
        Ok(())
    }
}
