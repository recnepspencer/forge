#[path = "persist/authority_primary.rs"]
mod authority_primary;
#[path = "persist/authority_support.rs"]
mod authority_support;
#[path = "persist/bulk.rs"]
mod bulk;
#[path = "persist/compatibility.rs"]
mod compatibility;
#[path = "persist/delta.rs"]
mod delta;
#[path = "persist/layout.rs"]
mod layout;
#[path = "persist/meta.rs"]
mod meta;
#[path = "persist/retention.rs"]
mod retention;
#[path = "persist/snapshot.rs"]
mod snapshot;
#[path = "persist/subscription_support.rs"]
mod subscription_support;
#[path = "persist/tiering.rs"]
mod tiering;

use crate::failure::StoreError;
use rusqlite::{Connection, Transaction};

use super::super::records::StoreState;
use super::helpers::sqlite_error;

pub(super) fn persist_state(
    connection: &mut Connection,
    state: &StoreState,
) -> Result<(), StoreError> {
    let transaction = connection.transaction().map_err(sqlite_error)?;
    clear_tables(&transaction)?;
    meta::persist_meta(&transaction, state)?;
    authority_primary::persist_authority_primary(&transaction, state)?;
    authority_support::persist_authority_support(&transaction, state)?;
    compatibility::persist_compatibility(&transaction, state)?;
    subscription_support::persist_subscription_support(&transaction, state)?;
    retention::persist_retention(&transaction, state)?;
    delta::persist_delta(&transaction, state)?;
    layout::persist_layout(&transaction, state)?;
    bulk::persist_bulk(&transaction, state)?;
    snapshot::persist_snapshot(&transaction, state)?;
    tiering::persist_tiering(&transaction, state)?;
    transaction.commit().map_err(sqlite_error)?;
    Ok(())
}

fn clear_tables(transaction: &Transaction<'_>) -> Result<(), StoreError> {
    transaction
        .execute_batch(
            "
            DELETE FROM authoritative_artifact_digests;
            DELETE FROM compatibility_manifest_records;
            DELETE FROM subscription_support_record_sets;
            DELETE FROM subscription_support_action_records;
            DELETE FROM subscription_support_maintenance_descriptor_records;
            DELETE FROM subscription_support_maintenance_debt_records;
            DELETE FROM subscription_support_counter_snapshot;
            DELETE FROM subscription_support_access_structure_state;
            DELETE FROM branch_head_records;
            DELETE FROM commit_support_summaries;
            DELETE FROM schema_support_records;
            DELETE FROM lineage_support_records;
            DELETE FROM durable_cursor_identity_records;
            DELETE FROM subscriber_checkpoint_records;
            DELETE FROM stable_basis_records;
            DELETE FROM compaction_product_records;
            DELETE FROM retention_basis_records;
            DELETE FROM retention_closure_records;
            DELETE FROM rebuild_debt_records;
            DELETE FROM maintenance_checkpoint_records;
            DELETE FROM maintenance_batch_records;
            DELETE FROM maintenance_execution_records;
            DELETE FROM maintenance_declaration_records;
            DELETE FROM maintenance_queue_summary_records;
            DELETE FROM maintenance_locality_summary_records;
            DELETE FROM maintenance_reservation_summary_records;
            DELETE FROM maintenance_resource_budget_summary_records;
            DELETE FROM maintenance_debt_summary_records;
            DELETE FROM branch_delta_layer_records;
            DELETE FROM branch_shared_base_records;
            DELETE FROM commit_parent_records;
            DELETE FROM commit_envelopes;
            DELETE FROM branch_records;
            DELETE FROM embedded_checkpoint_records;
            DELETE FROM milestone_6_layout_materialization_records;
            DELETE FROM milestone_6_commit_coupled_layout_seed_records;
            DELETE FROM milestone_6_scope_slice_membership_records;
            DELETE FROM milestone_6_chunk_membership_records;
            DELETE FROM milestone_6_structural_block_records;
            DELETE FROM bulk_program_identity_records;
            DELETE FROM frozen_bulk_manifest_records;
            DELETE FROM frozen_transform_basis_records;
            DELETE FROM frozen_transform_partition_records;
            DELETE FROM bulk_deterministic_plan_records;
            DELETE FROM bulk_progress_checkpoint_records;
            DELETE FROM bulk_chunk_witness_records;
            DELETE FROM program_chunk_witness_index_records;
            DELETE FROM snapshot_image_records;
            DELETE FROM snapshot_basis_records;
            DELETE FROM tier_transfer_records;
            DELETE FROM tier_residency_records;
            DELETE FROM tier_recall_records;
            DELETE FROM wal_records;
            DELETE FROM store_meta;
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
