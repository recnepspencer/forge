use crate::failure::StoreError;
use rusqlite::Transaction;

use super::super::super::records::StoreState;
use super::super::helpers::persist_bulk_json_record;

pub(super) fn persist_retention(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_json_records(
        transaction,
        "stable_basis_records",
        state.stable_basis_records.values(),
    )?;
    persist_json_records(
        transaction,
        "compaction_product_records",
        state.compaction_product_records.values(),
    )?;
    persist_json_records(
        transaction,
        "retention_basis_records",
        state.retention_basis_records.values(),
    )?;
    persist_json_records(
        transaction,
        "retention_closure_records",
        state.retention_closure_records.values(),
    )?;
    persist_json_records(
        transaction,
        "rebuild_debt_records",
        state.rebuild_debt_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_declaration_records",
        state.maintenance_declaration_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_execution_records",
        state.maintenance_execution_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_batch_records",
        state.maintenance_batch_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_checkpoint_records",
        state.maintenance_checkpoint_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_queue_summary_records",
        state.maintenance_queue_summary_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_locality_summary_records",
        state.maintenance_locality_summary_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_reservation_summary_records",
        state.maintenance_reservation_summary_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_resource_budget_summary_records",
        state.maintenance_resource_budget_summary_records.values(),
    )?;
    persist_json_records(
        transaction,
        "maintenance_debt_summary_records",
        state.maintenance_debt_summary_records.values(),
    )?;
    Ok(())
}

fn persist_json_records<'a, T, I>(
    transaction: &Transaction<'_>,
    table: &str,
    records: I,
) -> Result<(), StoreError>
where
    T: serde::Serialize + HasArtifactId + 'a,
    I: IntoIterator<Item = &'a T>,
{
    for record in records {
        persist_bulk_json_record(transaction, table, record.artifact_id(), Vec::new(), record)?;
    }
    Ok(())
}

trait HasArtifactId {
    fn artifact_id(&self) -> &str;
}

macro_rules! impl_artifact_id {
    ($($ty:path),+ $(,)?) => {
        $(impl HasArtifactId for $ty {
            fn artifact_id(&self) -> &str {
                &self.artifact_id
            }
        })+
    };
}

impl_artifact_id!(
    crate::backend::records::StableBasisRecord,
    crate::backend::records::CompactionProductRecord,
    crate::backend::records::RetentionBasisRecord,
    crate::backend::records::RetentionClosureRecord,
    crate::backend::records::RebuildDebtRecord,
    crate::backend::records::MaintenanceDeclarationRecord,
    crate::backend::records::MaintenanceExecutionRecord,
    crate::backend::records::MaintenanceBatchRecord,
    crate::backend::records::MaintenanceCheckpointRecord,
    crate::backend::records::MaintenanceQueueSummaryRecord,
    crate::backend::records::MaintenanceLocalitySummaryRecord,
    crate::backend::records::MaintenanceReservationSummaryRecord,
    crate::backend::records::MaintenanceResourceBudgetSummaryRecord,
    crate::backend::records::MaintenanceDebtSummaryRecord,
);
