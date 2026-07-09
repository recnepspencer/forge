use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{load_meta_u32, load_meta_u64};

pub(super) fn load_canonicalization_version(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    state.canonicalization_version = load_meta_u32(connection, "canonicalization_version")?
        .unwrap_or(state.canonicalization_version);
    Ok(())
}

pub(super) fn finalize_sequences(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    state.next_commit_sequence =
        load_meta_u64(connection, "next_commit_sequence")?.unwrap_or_else(|| {
            state
                .commit_envelopes
                .values()
                .map(|record| record.commit_sequence)
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_head_update_sequence = load_meta_u64(connection, "next_head_update_sequence")?
        .unwrap_or_else(|| {
            state
                .branch_head_records
                .values()
                .map(|record| record.head_update_sequence)
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_durable_mutation_id = load_meta_u64(connection, "next_durable_mutation_id")?
        .unwrap_or_else(|| {
            state
                .wal_records
                .values()
                .map(|record| record.durable_mutation_id.0)
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_snapshot_id = load_meta_u64(connection, "next_snapshot_id")?.unwrap_or_else(|| {
        state
            .snapshot_basis_records
            .keys()
            .max()
            .map(|value| value + 1)
            .unwrap_or(1)
    });
    state.next_branch_delta_layer_id = load_meta_u64(connection, "next_branch_delta_layer_id")?
        .unwrap_or_else(|| {
            state
                .branch_delta_layer_records
                .keys()
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    state.next_maintenance_declaration_order =
        load_meta_u64(connection, "next_maintenance_declaration_order")?.unwrap_or_else(|| {
            state
                .maintenance_declaration_records
                .values()
                .map(|record| record.created_order)
                .max()
                .unwrap_or(0)
        });
    state.next_maintenance_checkpoint_order =
        load_meta_u64(connection, "next_maintenance_checkpoint_order")?.unwrap_or_else(|| {
            state
                .maintenance_checkpoint_records
                .values()
                .map(|record| record.checkpoint_order)
                .max()
                .unwrap_or(0)
        });
    state.next_wal_sequence =
        load_meta_u64(connection, "next_wal_sequence")?.unwrap_or_else(|| {
            state
                .wal_records
                .keys()
                .max()
                .map(|value| value + 1)
                .unwrap_or(1)
        });
    Ok(())
}
