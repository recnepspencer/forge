use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::sqlite_error;

pub(super) fn persist_tiering(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.tier_residency_records.values() {
        transaction
            .execute(
                "
                INSERT INTO tier_residency_records(
                    artifact_key,
                    artifact_family,
                    canonical_residence,
                    canonical_replica_locator,
                    verification_label
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                ",
                params![
                    record.artifact_key,
                    record.artifact_family.label(),
                    record.canonical_residence.label(),
                    record.canonical_replica_locator,
                    record.verification_label,
                ],
            )
            .map_err(sqlite_error)?;
    }

    for record in state.tier_transfer_records.values() {
        transaction
            .execute(
                "
                INSERT INTO tier_transfer_records(
                    artifact_key,
                    artifact_family,
                    source_residence,
                    target_residence,
                    execution_origin,
                    source_replica_locator,
                    transferred_replica_locator,
                    verification_label,
                    cutover_completed
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ",
                params![
                    record.artifact_key,
                    record.artifact_family.label(),
                    record.source_residence.label(),
                    record.target_residence.label(),
                    record.execution_origin.label(),
                    record.source_replica_locator,
                    record.transferred_replica_locator,
                    record.verification_label,
                    if record.cutover_completed { 1i64 } else { 0i64 },
                ],
            )
            .map_err(sqlite_error)?;
    }

    for record in state.tier_recall_records.values() {
        transaction
            .execute(
                "
                INSERT INTO tier_recall_records(
                    coalescing_key,
                    artifact_family,
                    scope_class,
                    scope_key,
                    execution_origin,
                    artifact_key,
                    recall_cost_class,
                    amplification_budget,
                    completion_state
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ",
                params![
                    record.coalescing_key,
                    record.artifact_family.label(),
                    record.scope_class.label(),
                    record.scope_key,
                    record.execution_origin.label(),
                    record.artifact_key,
                    record.recall_cost_class.label(),
                    record.amplification_budget.label(),
                    record.completion_state.label(),
                ],
            )
            .map_err(sqlite_error)?;
    }

    Ok(())
}
