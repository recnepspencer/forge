use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::sqlite_error;

pub(super) fn persist_meta(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_meta_value(
        transaction,
        "canonicalization_version",
        state.canonicalization_version.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_commit_sequence",
        state.next_commit_sequence.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_head_update_sequence",
        state.next_head_update_sequence.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_durable_mutation_id",
        state.next_durable_mutation_id.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_snapshot_id",
        state.next_snapshot_id.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_branch_delta_layer_id",
        state.next_branch_delta_layer_id.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_maintenance_declaration_order",
        state.next_maintenance_declaration_order.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_maintenance_checkpoint_order",
        state.next_maintenance_checkpoint_order.to_string(),
    )?;
    persist_meta_value(
        transaction,
        "next_wal_sequence",
        state.next_wal_sequence.to_string(),
    )?;
    Ok(())
}

fn persist_meta_value(
    transaction: &Transaction<'_>,
    key: &str,
    value: String,
) -> Result<(), StoreError> {
    transaction
        .execute(
            "INSERT INTO store_meta(key, value) VALUES (?1, ?2)",
            params![key, value],
        )
        .map_err(sqlite_error)?;
    Ok(())
}
