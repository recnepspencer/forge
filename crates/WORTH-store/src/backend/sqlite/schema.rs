#[path = "schema/authority.rs"]
mod authority;
#[path = "schema/bulk_snapshot.rs"]
mod bulk_snapshot;
#[path = "schema/compatibility.rs"]
mod compatibility;
#[path = "schema/indexes.rs"]
mod indexes;
#[path = "schema/retention_layout.rs"]
mod retention_layout;
#[path = "schema/subscription_support.rs"]
mod subscription_support;
#[path = "schema/tiering.rs"]
mod tiering;

use crate::failure::StoreError;
use rusqlite::Connection;

use super::helpers::{
    ensure_branch_delta_layer_artifacts_column,
    migrate_milestone_6_commit_coupled_layout_seed_storage, sqlite_error,
};

pub(super) fn configure_connection(connection: &Connection) -> Result<(), StoreError> {
    connection
        .execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = FULL;
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}

pub(super) fn create_schema(connection: &Connection) -> Result<(), StoreError> {
    authority::create_authority_schema(connection)?;
    compatibility::create_compatibility_schema(connection)?;
    subscription_support::create_subscription_support_schema(connection)?;
    retention_layout::create_retention_layout_schema(connection)?;
    bulk_snapshot::create_bulk_snapshot_schema(connection)?;
    tiering::create_tiering_schema(connection)?;
    indexes::create_indexes(connection)?;
    migrate_milestone_6_commit_coupled_layout_seed_storage(connection)?;
    ensure_branch_delta_layer_artifacts_column(connection)?;
    Ok(())
}
