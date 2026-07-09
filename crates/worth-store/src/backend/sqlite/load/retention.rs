use crate::backend::records;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, sqlite_error};

pub(super) fn load_retention(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_json_records(
        connection,
        "stable_basis_records",
        &mut state.stable_basis_records,
    )?;
    load_json_records(
        connection,
        "compaction_product_records",
        &mut state.compaction_product_records,
    )?;
    load_json_records(
        connection,
        "retention_basis_records",
        &mut state.retention_basis_records,
    )?;
    load_json_records(
        connection,
        "retention_closure_records",
        &mut state.retention_closure_records,
    )?;
    load_json_records(
        connection,
        "rebuild_debt_records",
        &mut state.rebuild_debt_records,
    )?;
    load_json_records(
        connection,
        "maintenance_declaration_records",
        &mut state.maintenance_declaration_records,
    )?;
    load_json_records(
        connection,
        "maintenance_execution_records",
        &mut state.maintenance_execution_records,
    )?;
    load_json_records(
        connection,
        "maintenance_batch_records",
        &mut state.maintenance_batch_records,
    )?;
    load_json_records(
        connection,
        "maintenance_checkpoint_records",
        &mut state.maintenance_checkpoint_records,
    )?;
    load_json_records(
        connection,
        "maintenance_queue_summary_records",
        &mut state.maintenance_queue_summary_records,
    )?;
    load_json_records(
        connection,
        "maintenance_locality_summary_records",
        &mut state.maintenance_locality_summary_records,
    )?;
    load_json_records(
        connection,
        "maintenance_reservation_summary_records",
        &mut state.maintenance_reservation_summary_records,
    )?;
    load_json_records(
        connection,
        "maintenance_resource_budget_summary_records",
        &mut state.maintenance_resource_budget_summary_records,
    )?;
    load_json_records(
        connection,
        "maintenance_debt_summary_records",
        &mut state.maintenance_debt_summary_records,
    )?;
    Ok(())
}

fn load_json_records<T>(
    connection: &Connection,
    table: &str,
    target: &mut std::collections::BTreeMap<String, T>,
) -> Result<(), StoreError>
where
    T: serde::de::DeserializeOwned + HasArtifactId,
{
    let sql = format!("SELECT payload_json FROM {table} ORDER BY artifact_id");
    let mut statement = connection.prepare(&sql).map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| deserialize_json(row.get(0)?))
        .map_err(sqlite_error)?;
    for row in rows {
        let record: T = row.map_err(sqlite_error)?;
        target.insert(record.artifact_id().to_string(), record);
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
    records::StableBasisRecord,
    records::CompactionProductRecord,
    records::RetentionBasisRecord,
    records::RetentionClosureRecord,
    records::RebuildDebtRecord,
    records::MaintenanceDeclarationRecord,
    records::MaintenanceExecutionRecord,
    records::MaintenanceBatchRecord,
    records::MaintenanceCheckpointRecord,
    records::MaintenanceQueueSummaryRecord,
    records::MaintenanceLocalitySummaryRecord,
    records::MaintenanceReservationSummaryRecord,
    records::MaintenanceResourceBudgetSummaryRecord,
    records::MaintenanceDebtSummaryRecord,
);
